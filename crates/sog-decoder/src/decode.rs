use crate::error::{DecodeError, DecodeResult, ParseError, ParseResult, Result, UnzipResult};
use crate::metajson::MetaJsonType;
use crate::types::{Means, Quats, Scales, Sh0, ShN, SogDataV2, Splat};
use image_webp::WebPDecoder;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use zip::ZipArchive;
use zip::result::ZipError;

/// Unzip a zip file and return a HashMap of file names and their contents.
fn unzip(file_data: &[u8]) -> UnzipResult<HashMap<String, Vec<u8>>> {
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

fn parse_sog(files: HashMap<String, Vec<u8>>) -> ParseResult<SogDataV2> {
    let meta_bytes = files.get("meta.json").ok_or(ParseError::MetaJsonNotFound)?;

    let meta_json_string = str::from_utf8(meta_bytes)
        .map_err(|_| ParseError::InvalidMetaJson("encoding is not utf8".to_string()))?;

    let meta_json = serde_json::from_str::<MetaJsonType>(meta_json_string)
        .map_err(ParseError::DeserializeMetaJson)?;

    if meta_json.version != 2 {
        return Err(ParseError::InvalidMetaJson("version is not 2".to_string()));
    }

    let means_l_name = meta_json
        .means
        .files
        .get(0)
        .ok_or(ParseError::InvalidMetaJson(
            "missing means_l file name".to_string(),
        ))?;
    let means_u_name = meta_json
        .means
        .files
        .get(1)
        .ok_or(ParseError::InvalidMetaJson(
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

    let scales_name = meta_json
        .scales
        .files
        .get(0)
        .ok_or(ParseError::InvalidMetaJson(
            "missing scales file name".to_string(),
        ))?;
    let scales = Scales {
        codebook: meta_json.scales.codebook.as_slice().try_into()?,
        scales: files
            .get(scales_name)
            .ok_or(ParseError::ImageNotFound(scales_name.to_string()))?
            .clone(),
    };

    let quats_name = meta_json
        .quats
        .files
        .get(0)
        .ok_or(ParseError::InvalidMetaJson(
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
        .ok_or(ParseError::InvalidMetaJson(
            "missing sh0 file name".to_string(),
        ))?;
    let sh0 = Sh0 {
        codebook: meta_json.sh0.codebook.as_slice().try_into()?,
        sh0: files
            .get(sh0_name)
            .ok_or(ParseError::ImageNotFound(sh0_name.to_string()))?
            .clone(),
    };

    let sh_n = if let Some(sh_n) = meta_json.sh_n {
        let centroids_name = sh_n.files.get(0).ok_or(ParseError::InvalidMetaJson(
            "missing centroids file name".to_string(),
        ))?;
        let labels_name = sh_n.files.get(1).ok_or(ParseError::InvalidMetaJson(
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

pub fn unpack(file: &[u8]) -> Result<SogDataV2> {
    let files = unzip(file)?;
    let sog_data = parse_sog(files)?;
    Ok(sog_data)
}

fn decode_positions(means: &Means, count: usize) -> DecodeResult<Vec<f32>> {
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

    let mut positions = vec![0f32; count * 3];
    for i in 0..count {
        let pos_x = ((upper_pixels[i * 4 + 0] as u16) << 8) | (lower_pixels[i * 4 + 0] as u16);
        let pos_y = ((upper_pixels[i * 4 + 1] as u16) << 8) | (lower_pixels[i * 4 + 1] as u16);
        let pos_z = ((upper_pixels[i * 4 + 2] as u16) << 8) | (lower_pixels[i * 4 + 2] as u16);

        fn lerp(a: f32, b: f32, t: f32) -> f32 {
            a + t * (b - a)
        }
        fn unlog(x: f32) -> f32 {
            f32::signum(x) * (f32::exp(f32::abs(x)) - 1.0)
        }

        positions[i * 3 + 0] = unlog(lerp(mins.x, maxs.x, pos_x as f32 / 65535.0));
        positions[i * 3 + 1] = unlog(lerp(mins.y, maxs.y, pos_y as f32 / 65535.0));
        positions[i * 3 + 2] = unlog(lerp(mins.z, maxs.z, pos_z as f32 / 65535.0));
    }

    Ok(positions)
}

/// return: f32(x,y,z,w)
fn decode_rotations(quats: &Quats, count: usize) -> DecodeResult<Vec<f32>> {
    let cursor = Cursor::new(&quats.0);
    let mut decoder = WebPDecoder::new(cursor)?;
    let output_size = decoder
        .output_buffer_size()
        .ok_or_else(|| DecodeError::InvalidSize("cannot determine output size".to_string()))?;
    let mut pixels = vec![0u8; output_size];
    decoder.read_image(&mut pixels)?;

    fn to_comp(x: f32) -> f32 {
        (x / 255.0 - 0.5) * 2.0 / f32::sqrt(2.0)
    }

    let mut rotations = vec![0f32; count * 4];

    for i in 0..count {
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

        let q = match mode {
            0 => [d, a, b, c],
            1 => [a, d, b, c],
            2 => [a, b, d, c],
            3 => [a, b, c, d],
            _ => unreachable!(),
        };
        rotations[i * 4 + 0] = q[0];
        rotations[i * 4 + 1] = q[1];
        rotations[i * 4 + 2] = q[2];
        rotations[i * 4 + 3] = q[3];
    }

    Ok(rotations)
}

fn decode_scales(scales: &Scales, count: usize) -> DecodeResult<Vec<f32>> {
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

    let mut scales = vec![0f32; count * 3];
    for i in 0..count {
        scales[i * 3 + 0] = codebook.0[pixels[i * 4 + 0] as usize];
        scales[i * 3 + 1] = codebook.0[pixels[i * 4 + 1] as usize];
        scales[i * 3 + 2] = codebook.0[pixels[i * 4 + 2] as usize];
    }

    Ok(scales)
}

fn decode_color(sh0: &Sh0, count: usize) -> DecodeResult<Vec<f32>> {
    // const SH_C0: f32 = 0.28209479177387814; // SH_C0 = Y_0^0 = 1 / (2 * sqrt(pi))

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

    // https://github.com/playcanvas/splat-transform/blob/930a9aec511af3665240589b9cf1727d5dcd2eac/src/lib/readers/read-sog.ts#L174
    fn sigmoid_inv(y: f32) -> f32 {
        let e = y.clamp(1e-6, 1.0 - 1e-6);
        (e / (1.0 - e)).ln()
    }

    let mut colors = vec![0f32; count * 4];
    for i in 0..count {
        // colors[i * 4 + 0] = SH_C0 * codebook.0[pixels[i * 4 + 0] as usize] + 0.5;
        // colors[i * 4 + 1] = SH_C0 * codebook.0[pixels[i * 4 + 1] as usize] + 0.5;
        // colors[i * 4 + 2] = SH_C0 * codebook.0[pixels[i * 4 + 2] as usize] + 0.5;
        colors[i * 4 + 0] = codebook.0[pixels[i * 4 + 0] as usize];
        colors[i * 4 + 1] = codebook.0[pixels[i * 4 + 1] as usize];
        colors[i * 4 + 2] = codebook.0[pixels[i * 4 + 2] as usize];
        colors[i * 4 + 3] = sigmoid_inv(pixels[i * 4 + 3] as f32 / 255.0);
    }

    Ok(colors)
}

fn decode_sh_n(sh_n: &ShN, count: usize) -> DecodeResult<Vec<f32>> {
    let ShN {
        bands,
        codebook,
        centroids,
        labels,
        count: _,
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

    if centroids_pixels.len() % 3 != 0 || labels_pixels.len() % 4 != 0 {
        return Err(DecodeError::InvalidSize(
            "invalid image dimensions".to_string(),
        ));
    }

    // calc number of coefficients
    let coeff_count = match bands {
        1 => 3,
        2 => 8,
        3 => 15,
        _ => Err(DecodeError::InvalidData(format!(
            "invalid sh bands:{}",
            bands
        )))?,
    };

    let mut sh_n_s = vec![0f32; count * coeff_count * 3];
    for splat_index in 0..count {
        let palette_index = ((labels_pixels[splat_index * 4 + 0] as u16)
            | ((labels_pixels[splat_index * 4 + 1] as u16) << 8))
            as usize;

        for i in 0..3 {
            for coeff_index in 0..coeff_count {
                let index = (splat_index * 3 + i) * coeff_count + coeff_index;
                let index2 = (palette_index * coeff_count + coeff_index) * 3 + i;
                sh_n_s[index] = codebook.0[centroids_pixels[index2] as usize];
            }
        }
    }

    Ok(sh_n_s)
}

pub fn decode(sog_data: &SogDataV2) -> Result<Splat> {
    let SogDataV2 {
        means,
        quats,
        scales,
        sh0,
        sh_n,
        ..
    } = sog_data;

    let count = sog_data.count as usize;

    let splat = Splat {
        position: decode_positions(means, count)?,
        rotation: decode_rotations(quats, count)?,
        scale: decode_scales(scales, count)?,
        color: decode_color(sh0, count)?,
        sh: if let Some(s) = sh_n {
            Some(decode_sh_n(s, count)?)
        } else {
            None
        },
        count,
        antialias: sog_data.antialias,
        sh_degree: sh_n.as_ref().map(|s| s.bands as usize).unwrap_or(0usize),
    };

    Ok(splat)
}
