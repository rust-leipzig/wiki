//! Everything related to wikis error handling
extern crate glob;

use std::error::Error;
use std::{fmt, io};

#[derive(Default)]
/// The global Error type for wiki
pub struct WikiError {
    /// The type of the thrown error
    pub code: ErrorType,
    /// A further description for the error
    pub description: String,
    /// The cause for this error
    pub cause: Option<Box<Error>>,
}

/// Representation of an error case
impl WikiError {
    /// Creates a new `WikiError`
    pub fn new(code: ErrorType, description: &str) -> Self {
        WikiError {
            code: code,
            description: description.to_string(),
            cause: None,
        }
    }
}

impl fmt::Display for WikiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}] {}", self.code, self.description)
    }
}

impl fmt::Debug for WikiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for WikiError {
    fn description(&self) -> &str {
        &self.description
    }
}

/*macro_rules! from_error {
    ($err:ty, $err_v:ident, $err_t:path, $err_desc:expr) => {
        impl From<$err> for WikiError {
            fn from(err: $err) -> WikiError {
                WikiError {
                    code: $err_t,
                    description: $err_desc,
                    cause: Some(Box::new(err)),
                }
            }
        }
    }
}*/

impl From<io::Error> for WikiError {
    fn from(err: io::Error) -> WikiError {
        WikiError {
            code: ErrorType::Other,
            description: err.description().to_owned(),
            cause: Some(Box::new(err)),
        }
    }
}

impl From<glob::GlobError> for WikiError {
    fn from(err: glob::GlobError) -> WikiError {
        WikiError {
            code: ErrorType::GlobError,
            description: String::from("Handling of glob module failed.").to_owned(),
            cause: Some(Box::new(err)),
        }
    }
}

impl From<glob::PatternError> for WikiError {
    fn from(err: glob::PatternError) -> WikiError {
        WikiError {
            code: ErrorType::Other,
            description: String::from(err.msg).to_owned(),
            cause: Some(Box::new(err)),
        }
    }
}
/*from_error!(io::Error, ErrorType::Other, String::from(err.description()).to_owned());
from_error!(glob::GlobError, ErrorType::GlobError, String::from("Handling of glob module failed.").to_owned());
from_error!(glob::PatternError, ErrorType::Other, String::from(err.msg).to_owned());*/

#[derive(Debug, PartialEq)]
/// Error codes as indicator what happened
pub enum ErrorType {
    /// Everything worked fine
    Ok,
    /// The given path does not exist
    PathNotExisting,
    /// Handling of `glob` module failed
    GlobError,
    /// Pattern error while `glob` handling
    PatternError,
    /// Error within logging interface `mowl`
    LoggerError,
    /// An error not directly caused by `wiki` occured
    Other,
}

impl Default for ErrorType {
    fn default() -> ErrorType {ErrorType::Ok}
}
