use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
/// Represents an error that occurs parsing the frontmatter of a doc
pub struct ParseError {
    info: String,
    directory: String,
    error: Option<Box<dyn error::Error + Send + Sync + 'static>>,
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

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.error {
            None => None,
            Some(v) => v.source(),
        }
    }
}

impl ParseError {
    /// Creates a `ParseError`
    pub fn new(directory: &str, info: &str) -> Self {
        Self {
            directory: directory.to_owned(),
            info: info.to_owned(),
            error: None,
        }
    }
}

#[derive(Debug, PartialEq)]
/// Represents a document that was skipped over, typically due to being a draft
pub struct Skip {
    reason: String,
    directory: String,
}

impl fmt::Display for Skip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Skipping {}. {}", self.directory, self.reason)
    }
}

impl error::Error for Skip {
    fn description(&self) -> &str {
        &self.reason
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl Skip {
    /// Creates a `Skip`
    pub fn new(directory: &str, reason: &str) -> Self {
        Self {
            directory: directory.to_owned(),
            reason: reason.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
/// Represents an error that occurs constructing a path
pub struct PathError {
    reason: String,
    directory: String,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Skipping {}. {}", self.directory, self.reason)
    }
}

impl error::Error for PathError {
    fn description(&self) -> &str {
        &self.reason
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl PathError {
    /// Creates a `PathError`
    pub fn new(directory: &str, reason: &str) -> Self {
        Self {
            directory: directory.to_owned(),
            reason: reason.to_owned(),
        }
    }
}

#[derive(Debug)]
/// Represents the possible error results of trying to create a `PageIndex` from a document.
pub enum OperationResult {
    /// An IO error occurred.
    Io(io::Error),
    /// A parse error occurred.
    Parse(ParseError),
    /// It was skipped, possibly due to being a draft - also possibly because its frontmatter wasn't supported.
    Skip(Skip),
    /// There was an error constructing a path.
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
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types to a trait object `&Error`. This works because both error
            // types implement `Error`.
            OperationResult::Io(ref err) => Some(err),
            OperationResult::Parse(ref err) => Some(err),
            OperationResult::Skip(ref err) => Some(err),
            OperationResult::Path(ref err) => Some(err),
        }
    }
}
