use std::io::Cursor;

use zip::{ZipArchive, result::ZipError};

use std::io::Read;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

enum ArchivedSogFile {
    MetaJson(String),
    Image { filename: String, data: Vec<u8> },
}

fn extract_zip(zip_file_data: &[u8]) -> Result<Vec<ArchivedSogFile>, ZipError> {
    let cursor = Cursor::new(zip_file_data);
    let mut archive = ZipArchive::new(cursor)?;

    let mut files = Vec::new();
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index_raw(i)?;

        let name = zip_file.name().to_owned();

        let sog = if name == "meta.json" {
            let mut buf = String::new();
            zip_file.read_to_string(&mut buf)?;
            ArchivedSogFile::MetaJson(buf)
        } else {
            let mut buf = Vec::new();
            let _size = zip_file.read_to_end(&mut buf)?;
            ArchivedSogFile::Image {
                filename: name,
                data: buf,
            }
        };

        files.push(sog);
    }

    Ok(files)
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
