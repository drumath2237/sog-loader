use thiserror::Error;

#[derive(Debug, Error)]
pub enum SogDecodeError {
    #[error("Zip error: {0}")]
    Unzip(#[from] zip::result::ZipError),
}
