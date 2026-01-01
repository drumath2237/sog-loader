use crate::error::DecodeResult;
use crate::types::SogDataV2;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use zip::ZipArchive;
use zip::result::ZipError;

/// Unzip a zip file and return a HashMap of file names and their contents.
fn unzip(file_data: &[u8]) -> DecodeResult<HashMap<String, Vec<u8>>> {
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

fn construct_sog(files: HashMap<String, Vec<u8>>) -> DecodeResult<SogDataV2> {
    todo!()
}
