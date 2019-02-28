use std::io;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct MetaError {
    reason: String,
    count: usize
}

impl fmt::Display for MetaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program Error {}: {}", self.reason, self.count)
    }
}

impl error::Error for MetaError {
    fn description(&self) -> &str {
        &self.reason
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl MetaError {
    pub fn new (count: usize, reason: &str) -> Self {
        Self {
            count: count,
            reason: reason.to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum HugotoJsonError {
    IoError(io::Error),
    MetaError(MetaError),
    SerializationError(serde_json::error::Error)
}

impl From<io::Error> for HugotoJsonError {
    fn from(err: io::Error) -> Self {
        HugotoJsonError::IoError(err)
    }
}

impl From<MetaError> for HugotoJsonError {
    fn from(err: MetaError) -> Self {
        HugotoJsonError::MetaError(err)
    }
}

impl From<serde_json::error::Error> for HugotoJsonError {
    fn from(err: serde_json::error::Error) -> Self {
        HugotoJsonError::SerializationError(err)
    }
}

impl fmt::Display for HugotoJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HugotoJsonError::IoError(ref err) => write!(f, "IO error: {}", err),
            HugotoJsonError::MetaError(ref err) => write!(f, "Serialization error: {}", err),
            HugotoJsonError::SerializationError(ref err) => write!(f, "Serialization error: {}", err),
        }
    }
}

impl error::Error for HugotoJsonError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            HugotoJsonError::IoError(ref err) => err.description(),
            HugotoJsonError::MetaError(ref err) => err.description(),
            HugotoJsonError::SerializationError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types to a trait object `&Error`. This works because both error 
            // types implement `Error`.
            HugotoJsonError::IoError(ref err) => Some(err),
            HugotoJsonError::MetaError(ref err) => Some(err),
            HugotoJsonError::SerializationError(ref err) => Some(err)
        }
    }
}