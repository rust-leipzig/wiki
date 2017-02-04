//! Everything related to wikis error handling
use std::fmt;

#[derive(Default)]
/// The global Error type for wiki
pub struct WikiError {
    /// The type of the thrown error
    pub code: ErrorType,
    /// A further description for the error
    pub description: String,
}

/// Representation of an error case
impl WikiError {
    /// Creates a new `WikiError`
    pub fn new(code: ErrorType, description: &str) -> Self {
        WikiError {
            code: code,
            description: description.to_string(),
        }
    }
}

impl fmt::Display for WikiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}] {}", self.code, self.description)
    }
}

macro_rules! return_if_not_ok {
    ($retval:expr) => (
        if $retval.code != ErrorType::Ok {
            return $retval;
        }
    )
}

#[derive(Debug, PartialEq)]
/// Error codes as indicator what happened
pub enum ErrorType {
    /// Everything worked fine
    Ok,
    /// Failure while init process
    InitFailure,
    /// The given path does not exist
    PathNotExisting,
    /// The given path is not readable or accessible at all
    PathNotReadable,
    /// The given file can not be read in
    FileNotReadable,
    /// The conversion from file content to string failed
    BufferStringifyFailed,
    /// Something went wrong while conversion from markdown to HTML
    HtmlConversionFailed,
    /// An error without specific `ErrorType` occured
    Unknown,
}

impl Default for ErrorType {
    fn default() -> ErrorType {ErrorType::Ok}
}
