use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct Meta {
    reason: String,
    count: usize,
}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program Error {}: {}", self.reason, self.count)
    }
}

impl error::Error for Meta {
    fn description(&self) -> &str {
        &self.reason
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl Meta {
    pub fn new(count: usize, reason: &str) -> Self {
        Self {
            count: count,
            reason: reason.to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum HugotoJsonError {
    Io(io::Error),
    Meta(Meta),
    Serialization(serde_json::error::Error),
}

impl From<io::Error> for HugotoJsonError {
    fn from(err: io::Error) -> Self {
        HugotoJsonError::Io(err)
    }
}

impl From<Meta> for HugotoJsonError {
    fn from(err: Meta) -> Self {
        HugotoJsonError::Meta(err)
    }
}

impl From<serde_json::error::Error> for HugotoJsonError {
    fn from(err: serde_json::error::Error) -> Self {
        HugotoJsonError::Serialization(err)
    }
}

impl fmt::Display for HugotoJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HugotoJsonError::Io(ref err) => write!(f, "IO error: {}", err),
            HugotoJsonError::Meta(ref err) => write!(f, "Serialization error: {}", err),
            HugotoJsonError::Serialization(ref err) => write!(f, "Serialization error: {}", err),
        }
    }
}

impl error::Error for HugotoJsonError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            HugotoJsonError::Io(ref err) => err.description(),
            HugotoJsonError::Meta(ref err) => err.description(),
            HugotoJsonError::Serialization(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types to a trait object `&Error`. This works because both error
            // types implement `Error`.
            HugotoJsonError::Io(ref err) => Some(err),
            HugotoJsonError::Meta(ref err) => Some(err),
            HugotoJsonError::Serialization(ref err) => Some(err),
        }
    }
}
