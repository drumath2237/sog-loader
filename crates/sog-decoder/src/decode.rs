use crate::error::{DecodeError, DecodeResult, Error, ParseError, Result};
use crate::metajson::MetaJsonType;
use crate::types::{
    Color3, Color4, Means, Quaternion, Quats, Scales, Sh0, ShN, SogDataV2, Splat, Vector3,
};
use image_webp::WebPDecoder;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use zip::ZipArchive;
use zip::result::ZipError;

/// Unzip a zip file and return a HashMap of file names and their contents.
pub fn unzip(file_data: &[u8]) -> Result<HashMap<String, Vec<u8>>> {
    let cursor = Cursor::new(file_data);
    let mut archive = ZipArchive::new(cursor)?;
    let mut files = HashMap::new();

    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i)?;
        let mut buf = Vec::with_capacity(zip_file.size() as usize);
        let _size = zip_file.read_to_end(&mut buf).map_err(ZipError::Io)?;
        files.insert(zip_file.name().to_owned(), buf);
    }

    Ok(files)
}

pub fn parse_sog(files: HashMap<String, Vec<u8>>) -> Result<SogDataV2> {
    let meta_bytes = files.get("meta.json").ok_or(Error::MetaJsonNotFound)?;

    let meta_json_string = String::from_utf8(meta_bytes.clone())
        .map_err(|_| Error::InvalidMetaJson("encoding is not utf8".to_string()))?;

    let meta_json = serde_json::from_str::<MetaJsonType>(meta_json_string.as_str())
        .map_err(Error::DeserializeMetaJson)?;

    let means_l_name = meta_json.means.files.get(0).ok_or(Error::InvalidMetaJson(
        "missing means_l file name".to_string(),
    ))?;
    let means_u_name = meta_json.means.files.get(1).ok_or(Error::InvalidMetaJson(
        "missing means_u file name".to_string(),
    ))?;
    let means = Means {
        mins: meta_json.means.mins.try_into()?,
        maxs: meta_json.means.maxs.try_into()?,
        means_l: files
            .get(means_l_name)
            .ok_or(ParseError::ImageNotFound(means_l_name.to_string()))?
            .clone(),
        means_u: files
            .get(means_u_name)
            .ok_or(ParseError::ImageNotFound(means_u_name.to_string()))?
            .clone(),
    };

    let scales_name = meta_json.scales.files.get(0).ok_or(Error::InvalidMetaJson(
        "missing scales file name".to_string(),
    ))?;
    let scales = Scales {
        codebook: meta_json.scales.codebook.as_slice().try_into()?,
        scales: files
            .get(scales_name)
            .ok_or(ParseError::ImageNotFound(scales_name.to_string()))?
            .clone(),
    };

    let quats_name = meta_json.quats.files.get(0).ok_or(Error::InvalidMetaJson(
        "missing quats file name".to_string(),
    ))?;
    let quats = Quats(
        files
            .get(quats_name)
            .ok_or(ParseError::ImageNotFound(quats_name.to_string()))?
            .clone(),
    );

    let sh0_name = meta_json
        .sh0
        .files
        .get(0)
        .ok_or(Error::InvalidMetaJson("missing sh0 file name".to_string()))?;
    let sh0 = Sh0 {
        codebook: meta_json.sh0.codebook.as_slice().try_into()?,
        sh0: files
            .get(sh0_name)
            .ok_or(ParseError::ImageNotFound(sh0_name.to_string()))?
            .clone(),
    };

    let sh_n = if let Some(sh_n) = meta_json.sh_n {
        let centroids_name = sh_n.files.get(0).ok_or(Error::InvalidMetaJson(
            "missing centroids file name".to_string(),
        ))?;
        let labels_name = sh_n.files.get(1).ok_or(Error::InvalidMetaJson(
            "missing labels file name".to_string(),
        ))?;
        Some(ShN {
            count: sh_n.count,
            bands: sh_n.bands,
            codebook: sh_n.codebook.as_slice().try_into()?,
            centroids: files
                .get(centroids_name)
                .ok_or(ParseError::ImageNotFound(centroids_name.to_string()))?
                .clone(),
            labels: files
                .get(labels_name)
                .ok_or(ParseError::ImageNotFound(labels_name.to_string()))?
                .clone(),
        })
    } else {
        None
    };

    Ok(SogDataV2 {
        count: meta_json.count,
        antialias: meta_json.antialias.unwrap_or(false),
        means,
        quats,
        scales,
        sh0,
        sh_n,
    })
}

fn decode_positions(means: &Means) -> DecodeResult<Vec<Vector3>> {
    let Means {
        mins,
        maxs,
        means_l,
        means_u,
    } = means;

    let cursor = Cursor::new(means_l);
    let mut decoder = WebPDecoder::new(cursor)?;
    let output_size = decoder.output_buffer_size().ok_or_else(|| {
        DecodeError::InvalidSize("Failed to get output buffer size of WebP image".to_string())
    })?;
    let mut lower_pixels = vec![0u8; output_size];
    decoder.read_image(&mut lower_pixels)?;

    let cursor = Cursor::new(means_u);
    let mut decoder = WebPDecoder::new(cursor)?;
    let output_size = decoder.output_buffer_size().ok_or_else(|| {
        DecodeError::InvalidSize("Failed to get output buffer size of WebP image".to_string())
    })?;
    let mut upper_pixels = vec![0u8; output_size];
    decoder.read_image(&mut upper_pixels)?;

    // sanitize
    if lower_pixels.len() != upper_pixels.len() {
        return Err(DecodeError::InvalidSize(
            "Lower and upper pixels have different length".to_string(),
        ));
    } else if lower_pixels.len() % 4 != 0 {
        return Err(DecodeError::InvalidSize(format!(
            "lower image size cannot be divided by 4: {}",
            lower_pixels.len()
        )));
    } else if upper_pixels.len() % 4 != 0 {
        return Err(DecodeError::InvalidSize(format!(
            "upper image size cannot be divided by 4: {}",
            upper_pixels.len()
        )));
    }

    let mut positions = vec![Vector3::default(); lower_pixels.len() / 4];
    for i in 0..positions.len() {
        let pos_x = ((upper_pixels[i * 4 + 0] as u16) << 8) | (lower_pixels[i * 4 + 0] as u16);
        let pos_y = ((upper_pixels[i * 4 + 1] as u16) << 8) | (lower_pixels[i * 4 + 1] as u16);
        let pos_z = ((upper_pixels[i * 4 + 2] as u16) << 8) | (lower_pixels[i * 4 + 2] as u16);

        fn lerp(a: f32, b: f32, t: f32) -> f32 {
            a + t * (b - a)
        }
        fn unlog(x: f32) -> f32 {
            f32::signum(x) * (f32::exp(f32::abs(x)) - 1.0)
        }

        positions[i] = Vector3::new(
            unlog(lerp(mins.x, maxs.x, pos_x as f32 / 65535.0)),
            unlog(lerp(mins.y, maxs.y, pos_y as f32 / 65535.0)),
            unlog(lerp(mins.z, maxs.z, pos_z as f32 / 65535.0)),
        );
    }

    Ok(positions)
}

fn decode_rotations(quats: &Quats) -> DecodeResult<Vec<Quaternion>> {
    let cursor = Cursor::new(&quats.0);
    let mut decoder = WebPDecoder::new(cursor)?;
    let output_size = decoder
        .output_buffer_size()
        .ok_or_else(|| DecodeError::InvalidSize("cannot determine output size".to_string()))?;
    let mut pixels = vec![0u8; output_size];
    decoder.read_image(&mut pixels)?;

    fn to_comp(x: f32) -> f32 {
        (x / 255.0 - 0.5) * 0.2 / f32::sqrt(2.0)
    }

    let mut rotations = vec![Quaternion::default(); pixels.len() / 4];

    println!("pixels len: {}", pixels.len());
    println!("rotations len: {}", rotations.len());
    
    for i in 0..rotations.len() {
        let a = to_comp(pixels[i * 4 + 0] as f32);
        let b = to_comp(pixels[i * 4 + 1] as f32);
        let c = to_comp(pixels[i * 4 + 2] as f32);
        let m = pixels[i * 4 + 3];

        if m < 252 {
            return Err(DecodeError::InvalidData(format!(
                "invalid rotation mode(m<252): {}, index: {}",
                m, i
            )));
        }

        let mode = match m - 252 {
            0u8 => Ok(0u8),
            1u8 => Ok(1u8),
            2u8 => Ok(2u8),
            3u8 => Ok(3u8),
            _ => Err(DecodeError::InvalidData(format!(
                "invalid rotation mode: {}",
                pixels[i * 4 + 3] - 252
            ))),
        }?;
        let d = f32::sqrt(f32::max(0.0, 1.0 - a * a - b * b - c * c));

        rotations[i] = match mode {
            0 => Quaternion::new(d, a, b, c),
            1 => Quaternion::new(a, d, b, c),
            2 => Quaternion::new(a, b, d, c),
            3 => Quaternion::new(a, b, c, d),
            _ => unreachable!(),
        }
    }

    Ok(rotations)
}

fn decode_scales(scales: &Scales) -> DecodeResult<Vec<Vector3>> {
    let Scales { codebook, scales } = scales;

    let cursor = Cursor::new(scales);
    let mut decoder = WebPDecoder::new(cursor)?;
    let output_size = decoder
        .output_buffer_size()
        .ok_or_else(|| DecodeError::InvalidSize("cannot determine output size".to_string()))?;
    let mut pixels = vec![0u8; output_size];
    decoder.read_image(&mut pixels)?;

    if pixels.len() % 4 != 0 {
        return Err(DecodeError::InvalidData(format!(
            "scale image size cannot be divided by 4: {}",
            pixels.len()
        )));
    }

    let mut scales = vec![Vector3::default(); pixels.len() / 4];
    for i in 0..scales.len() {
        scales[i] = Vector3::new(
            codebook.0[pixels[i * 4 + 0] as usize],
            codebook.0[pixels[i * 4 + 1] as usize],
            codebook.0[pixels[i * 4 + 2] as usize],
        );
    }

    Ok(scales)
}

fn decode_color(sh0: &Sh0) -> DecodeResult<Vec<Color4>> {
    const SH_C0: f32 = 0.28209479177387814; // SH_C0 = Y_0^0 = 1 / (2 * sqrt(pi))

    let Sh0 { codebook, sh0 } = sh0;

    let cursor = Cursor::new(sh0);
    let mut decoder = WebPDecoder::new(cursor)?;
    let output_size = decoder
        .output_buffer_size()
        .ok_or_else(|| DecodeError::InvalidSize("cannot determine output size".to_string()))?;
    let mut pixels = vec![0u8; output_size];
    decoder.read_image(&mut pixels)?;

    if pixels.len() % 4 != 0 {
        return Err(DecodeError::InvalidData(format!(
            "color image size cannot be divided by 4: {}",
            pixels.len()
        )));
    }

    let mut colors = vec![Color4::default(); pixels.len() / 4];
    for i in 0..colors.len() {
        colors[i] = Color4::new(
            SH_C0 * codebook.0[pixels[i * 4 + 0] as usize],
            SH_C0 * codebook.0[pixels[i * 4 + 1] as usize],
            SH_C0 * codebook.0[pixels[i * 4 + 2] as usize],
            pixels[i * 4 + 3] as f32 / 255.0,
        )
    }

    Ok(colors)
}

fn decode_sh_n(sh_n: &ShN) -> DecodeResult<Vec<Color3>> {
    let ShN {
        count: _,
        bands,
        codebook,
        centroids,
        labels,
    } = sh_n;

    if *bands <= 0 || *bands >= 4 {
        return Err(DecodeError::InvalidSize(format!(
            "invalid bands count: {}",
            bands
        )));
    }

    let cursor = Cursor::new(centroids);
    let mut decoder = WebPDecoder::new(cursor)?;
    let output_size = decoder
        .output_buffer_size()
        .ok_or_else(|| DecodeError::InvalidSize("cannot determine output size".to_string()))?;
    let mut centroids_pixels = vec![0u8; output_size];
    decoder.read_image(&mut centroids_pixels)?;

    let cursor = Cursor::new(labels);
    let mut decoder = WebPDecoder::new(cursor)?;
    let output_size = decoder
        .output_buffer_size()
        .ok_or_else(|| DecodeError::InvalidSize("cannot determine output size".to_string()))?;
    let mut labels_pixels = vec![0u8; output_size];
    decoder.read_image(&mut labels_pixels)?;

    if centroids_pixels.len() % 4 != 0 || labels_pixels.len() % 4 != 0 {
        return Err(DecodeError::InvalidSize(
            "invalid image dimensions".to_string(),
        ));
    }

    let mut palette_indices: Vec<u16> = vec![0u16; labels_pixels.len() / 4];
    for i in 0..palette_indices.len() {
        palette_indices[i] =
            (labels_pixels[i * 4 + 0] as u16) | ((labels_pixels[i * 4 + 1] as u16) << 8);
    }

    // calc number of coefficients
    let coeff_count = match bands {
        1 => 3,
        2 => 8,
        3 => 15,
        _ => unreachable!(),
    };

    let mut sh_n_s: Vec<Color3> = vec![Color3::default(); palette_indices.len() * coeff_count];
    for i in 0..palette_indices.len() {
        let palette_index = palette_indices[i] as usize;
        for coeff_index in 0..coeff_count {
            let index = i * coeff_count + coeff_index;
            sh_n_s[index] = Color3::new(
                codebook.0[centroids_pixels[palette_index * 4 + 0] as usize],
                codebook.0[centroids_pixels[palette_index * 4 + 1] as usize],
                codebook.0[centroids_pixels[palette_index * 4 + 2] as usize],
            )
        }
    }

    Ok(sh_n_s)
}

pub fn decode_sog(sog_data: &SogDataV2) -> Result<Splat> {
    let SogDataV2 {
        means,
        quats,
        scales,
        sh0,
        sh_n,
        count,
        antialias,
    } = sog_data;

    let splat = Splat {
        position: decode_positions(means)?,
        rotation: decode_rotations(quats)?,
        scale: decode_scales(scales)?,
        color: decode_color(sh0)?,
        sh_n: if let Some(s) = sh_n {
            Some(decode_sh_n(&s)?)
        } else {
            None
        },
        count: count.clone() as usize,
        antialias: antialias.clone(),
    };

    Ok(splat)
}
