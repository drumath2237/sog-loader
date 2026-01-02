use crate::error::{DecodeError, DecodeResult};
use crate::metajson::MetaJsonType;
use crate::types::{Means, ParseError, Quats, Scales, Sh0, ShN, SogDataV2};
use std::collections::HashMap;
use std::io::{Cursor, Read};
use zip::ZipArchive;
use zip::result::ZipError;

/// Unzip a zip file and return a HashMap of file names and their contents.
pub fn unzip(file_data: &[u8]) -> DecodeResult<HashMap<String, Vec<u8>>> {
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

pub fn parse_sog(files: HashMap<String, Vec<u8>>) -> DecodeResult<SogDataV2> {
    let meta_bytes = files
        .get("meta.json")
        .ok_or(DecodeError::MetaJsonNotFound)?;

    let meta_json_string = String::from_utf8(meta_bytes.clone())
        .map_err(|_| DecodeError::InvalidMetaJson("encoding is not utf8".to_string()))?;

    let meta_json = serde_json::from_str::<MetaJsonType>(meta_json_string.as_str())
        .map_err(DecodeError::DeserializeMetaJson)?;

    let means_l_name = meta_json
        .means
        .files
        .get(0)
        .ok_or(DecodeError::InvalidMetaJson(
            "missing means_l file name".to_string(),
        ))?;
    let means_u_name = meta_json
        .means
        .files
        .get(1)
        .ok_or(DecodeError::InvalidMetaJson(
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
        .ok_or(DecodeError::InvalidMetaJson(
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
        .ok_or(DecodeError::InvalidMetaJson(
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
        .ok_or(DecodeError::InvalidMetaJson(
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
        let centroids_name = sh_n.files.get(0).ok_or(DecodeError::InvalidMetaJson(
            "missing centroids file name".to_string(),
        ))?;
        let labels_name = sh_n.files.get(1).ok_or(DecodeError::InvalidMetaJson(
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
