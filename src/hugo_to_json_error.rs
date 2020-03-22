use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HugotoJsonError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("An error occurred. Failed to process {total} files.")]
    Meta { total: usize },
    #[error("An error occurred serializing the index: {0}")]
    Serialization(#[from] serde_json::error::Error),
    #[error("Unknown error")]
    Unknown,
}
