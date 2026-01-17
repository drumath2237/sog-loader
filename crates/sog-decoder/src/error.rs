use image_webp::DecodingError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Unzip(#[from] UnzipError),

    #[error("{0}")]
    SogParse(#[from] ParseError),

    #[error("{0}")]
    SogDecode(#[from] DecodeError),
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum UnzipError {
    #[error("Zip error: {0}")]
    Unzip(#[from] zip::result::ZipError),
}

pub type UnzipResult<T> = core::result::Result<T, UnzipError>;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("meta.json not found")]
    MetaJsonNotFound,
    #[error("meta.json is invalid data: {0}")]
    InvalidMetaJson(String),
    #[error("Deserialize error: {0}")]
    DeserializeMetaJson(#[from] serde_json::Error),
    #[error("invalid vector data")]
    ParseVector(String),
    #[error("invalid codebook length")]
    ParseCodebook(String),
    #[error("image file not found: {0}")]
    ImageNotFound(String),
}

pub type ParseResult<T> = core::result::Result<T, ParseError>;

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("Image not found: {0}")]
    DecodeImage(#[from] DecodingError),
    #[error("Invalid size: {0}")]
    InvalidSize(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub type DecodeResult<T> = core::result::Result<T, DecodeError>;
