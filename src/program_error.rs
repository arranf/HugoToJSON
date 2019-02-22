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
pub enum ProgramError {
    IoError(io::Error),
    MetaError(MetaError),
    SerializationError(serde_json::error::Error)
}

impl From<io::Error> for ProgramError {
    fn from(err: io::Error) -> Self {
        ProgramError::IoError(err)
    }
}

impl From<MetaError> for ProgramError {
    fn from(err: MetaError) -> Self {
        ProgramError::MetaError(err)
    }
}

impl From<serde_json::error::Error> for ProgramError {
    fn from(err: serde_json::error::Error) -> Self {
        ProgramError::SerializationError(err)
    }
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProgramError::IoError(ref err) => write!(f, "IO error: {}", err),
            ProgramError::MetaError(ref err) => write!(f, "Serialization error: {}", err),
            ProgramError::SerializationError(ref err) => write!(f, "Serialization error: {}", err),
        }
    }
}

impl error::Error for ProgramError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            ProgramError::IoError(ref err) => err.description(),
            ProgramError::MetaError(ref err) => err.description(),
            ProgramError::SerializationError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types to a trait object `&Error`. This works because both error 
            // types implement `Error`.
            ProgramError::IoError(ref err) => Some(err),
            ProgramError::MetaError(ref err) => Some(err),
            ProgramError::SerializationError(ref err) => Some(err)
        }
    }
}