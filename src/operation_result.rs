use std::io;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    info: String,
    directory: String,
    error: Option<Box<dyn error::Error+Send+Sync>>
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error at {}. {}. Skipping.", self.directory, self.info)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        &self.info
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match &self.error {
            None => None,
            Some(v) => v.cause()
        }
    }
}

impl ParseError {
    pub fn new (directory: &str, info: &str) -> Self {
        Self {
            directory: directory.to_owned(),
            info: info.to_owned(),
            error: None
        }
    }
}

#[derive(Debug)]
pub struct Skip {
    reason: String,
    directory: String
}

impl fmt::Display for Skip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Skiping {}. {}", self.directory, self.reason)
    }
}

impl error::Error for Skip {
    fn description(&self) -> &str {
        &self.reason
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl Skip {
    pub fn new (directory: &str, reason: &str) -> Self {
        Self {
            directory: directory.to_owned(),
            reason: reason.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct PathError {
    reason: String,
    directory: String
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Skiping {}. {}", self.directory, self.reason)
    }
}

impl error::Error for PathError {
    fn description(&self) -> &str {
        &self.reason
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl PathError {
    pub fn new (directory: &str, reason: &str) -> Self {
        Self {
            directory: directory.to_owned(),
            reason: reason.to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum OperationResult {
    Io(io::Error),
    Parse(ParseError),
    Skip(Skip),
    Path(PathError),
}

impl From<io::Error> for OperationResult {
    fn from(err: io::Error) -> Self {
        OperationResult::Io(err)
    }
}

impl From<Skip> for OperationResult {
    fn from(err: Skip) -> Self {
        OperationResult::Skip(err)
    }
}

impl From<ParseError> for OperationResult {
    fn from(err: ParseError) -> Self {
        OperationResult::Parse(err)
    }
}

impl From<PathError> for OperationResult {
    fn from(err: PathError) -> Self {
        OperationResult::Path(err)
    }
}

impl fmt::Display for OperationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            OperationResult::Io(ref err) => write!(f, "IO error: {}", err),
            OperationResult::Parse(ref err) => write!(f, "Parse error: {}", err),
            OperationResult::Skip(ref err) => write!(f, "Skipped: {}", err),
            OperationResult::Path(ref err) => write!(f, "Path manipulaiton error: {}", err),
        }
    }
}

impl error::Error for OperationResult {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            OperationResult::Io(ref err) => err.description(),
            OperationResult::Parse(ref err) => err.description(),
            OperationResult::Skip(ref err) => err.description(),
            OperationResult::Path(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types to a trait object `&Error`. This works because both error 
            // types implement `Error`.
            OperationResult::Io(ref err) => Some(err),
            OperationResult::Parse(ref err) => Some(err),
            OperationResult::Skip(ref err) => Some(err),
            OperationResult::Path(ref err) => Some(err)
        }
    }
}