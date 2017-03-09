//! Everything related to wikis error handling
use std::error::Error;
use std::{fmt, io};
use glob;
use iron::error::HttpError;

/// Common Wiki Result type
pub type WikiResult<T> = Result<T, WikiError>;

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

macro_rules! from_error {
    ($($p:ty,)*) => (
        $(impl From<$p> for WikiError {
            fn from(err: $p) -> Self {
                WikiError {
                    code: ErrorType::Other,
                    description: err.description().to_owned(),
                    cause: Some(Box::new(err)),
                }
            }
        })*
    )
}

from_error! {
    io::Error,
    glob::GlobError,
    glob::PatternError,
    HttpError,
}

#[derive(Debug, PartialEq)]
/// Error codes as indicator what happened
pub enum ErrorType {
    /// Everything worked fine
    Ok,

    /// A CLI parameter is missing
    CliParameterMissing,

    /// The given path does not exist
    PathNotExisting,

    /// Error within logging interface `mowl`
    LoggerError,

    /// An error not directly caused by `wiki` occured
    Other,
}

impl Default for ErrorType {
    fn default() -> ErrorType {
        ErrorType::Ok
    }
}

macro_rules! bail {
    ($p:expr, $($fmt:tt)*) => (
        #[cfg_attr(feature = "cargo-clippy", allow(useless_format))]
        return Err(::error::WikiError::new($p, &format!($($fmt)*)))
    )
}
