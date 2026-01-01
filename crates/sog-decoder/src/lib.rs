use std::{collections::HashMap, io::Cursor};

use zip::{ZipArchive, result::ZipError};

use std::io::Read;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Default)]
struct ArchivedSogFile {
    pub meta_json: Option<String>,
    pub image_files: HashMap<String, Vec<u8>>,
}

fn extract_zip(zip_file_data: &[u8]) -> Result<ArchivedSogFile, ZipError> {
    let cursor = Cursor::new(zip_file_data);
    let mut archive = ZipArchive::new(cursor)?;

    let mut archived_sog = ArchivedSogFile::default();

    for i in 0..archive.len() {
        let mut zip_file = archive.by_index_raw(i)?;

        if zip_file.name() == "meta.json" {
            let mut buf = String::new();
            zip_file.read_to_string(&mut buf)?;
            archived_sog.meta_json = Some(buf);
        } else {
            let mut buf = Vec::new();
            let _size = zip_file.read_to_end(&mut buf)?;
            archived_sog
                .image_files
                .insert(zip_file.name().to_owned(), buf);
        };
    }

    Ok(archived_sog)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
