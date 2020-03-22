use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
/// Represents the different possible error states that can arise from trying to produce an index.
pub enum HugotoJsonError {
    /// An IO error i.e. an error reading, writing or seeking most likely from the OS.
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    /// Represents a case of one or more errors traversing and reading files in the contents directory.
    #[error("An error occurred. Failed to process {total} files.")]
    Meta {
        /// The number of errors that occurred.
        total: usize,
    },
    /// An error trying to serialize to JSON.
    #[error("An error occurred serializing the index: {0}")]
    Serialization(#[from] serde_json::error::Error),
    /// A catchall for all unknown errors.
    #[error("Unknown error")]
    Unknown,
}
