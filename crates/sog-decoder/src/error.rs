use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Zip error: {0}")]
    Unzip(#[from] zip::result::ZipError),
}

pub type DecodeResult<T> = Result<T, DecodeError>;
