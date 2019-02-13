
use std::io;
use std::error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct ParseError<'a> {
    info: String,
    directory: String,
    error: Option<&'a error::Error>
}

impl fmt::Display for ParseError<'static> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error at {}. {}. Skipping.", self.directory, self.info)
    }
}

impl error::Error for ParseError<'static> {
    fn description(&self) -> &str {
        &self.to_string()
    }

    fn cause(&self) -> Option<&error::Error> {
        self.error
    }
}

impl ParseError<'static> {
    pub fn new<'a> (directory: String, info: &str, error: Option<&'a error::Error>) -> ParseError<'a> {
        ParseError {
            directory: directory.to_owned(),
            info: info.to_owned(),
            error
        }
    }
}

impl fmt::Display for CliError<'static> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for CliError<'static> {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            CliError::Io(ref err) => err.description(),
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which conflicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            CliError::Parse(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
        }
    }
}


#[derive(Debug)]
enum CliError<'a> {
    Io(io::Error),
    Parse(ParseError<'a>)
    // TODO: Some sort of SkipError
}