use crate::types::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Zip error: {0}")]
    Unzip(#[from] zip::result::ZipError),
    #[error("meta.json not found")]
    MetaJsonNotFound,
    #[error("meta.json is invalid data: {0}")]
    InvalidMetaJson(String),
    #[error("Deserialize error: {0}")]
    DeserializeMetaJson(#[from] serde_json::Error),
    #[error("Sog parse error: {0}")]
    SogParse(#[from] ParseError),
}

pub type DecodeResult<T> = Result<T, DecodeError>;
