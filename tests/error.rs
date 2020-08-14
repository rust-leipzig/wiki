extern crate wikilib;

use wikilib::error::*;
use std::error::Error as StdError;
use std::io;

static ERROR_STR: &str = "Something went wrong here.";

#[test]
fn io_error_to_wiki_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, ERROR_STR);
    let wiki_error: Error = io_error.into();
    assert_eq!(wiki_error.to_string(), ERROR_STR.to_string());
}
