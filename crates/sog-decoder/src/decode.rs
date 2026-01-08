use crate::error::{DecodeError, DecodeResult, Error, ParseError, Result};
use crate::metajson::MetaJsonType;
use crate::types::{Means, Quaternion, Quats, Scales, Sh0, ShN, SogDataV2, Vector3};
use image_webp::WebPDecoder;
use serde::de::Unexpected::Float;
use std::collections::HashMap;
use std::fmt::format;
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

pub fn decode_positions(means: &Means) -> DecodeResult<Vec<Vector3>> {
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
        let pos_x = (((upper_pixels[i * 4 + 0]) << 8) as u16) | (lower_pixels[i * 4 + 0] as u16);
        let pos_y = (((upper_pixels[i * 4 + 1]) << 8) as u16) | (lower_pixels[i * 4 + 1] as u16);
        let pos_z = (((upper_pixels[i * 4 + 2]) << 8) as u16) | (lower_pixels[i * 4 + 2] as u16);

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

    fn toComp(x: f32) -> f32 {
        (x / 255.0 - 0.5) * 0.2 / f32::sqrt(2.0)
    }

    let mut rotations = vec![Quaternion::default(); pixels.len() / 4];
    for i in 0..rotations.len() {
        let a = toComp(pixels[i * 4 + 0] as f32);
        let b = toComp(pixels[i * 4 + 1] as f32);
        let c = toComp(pixels[i * 4 + 2] as f32);
        let m = pixels[i * 4 + 3];
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

fn decode_scales(scales: &Scales) -> DecodeResult<Vec<f32>> {
    todo!()
}
