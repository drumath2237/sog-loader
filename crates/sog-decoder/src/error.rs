use thiserror::Error;

#[derive(Debug, Error)]
pub enum SogDecodeError {
    #[error("Zip error: {0}")]
    UnzipError(#[from] zip::result::ZipError),
}
