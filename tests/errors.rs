extern crate wikilib;
use wikilib::WikiError;
use std::io;

#[test]
fn into_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "Not found");
    let wiki_error: WikiError = io_error.into();
    assert_eq!(wiki_error.description, "Not found");
}

#[test]
fn from_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "Not found");
    let wiki_error = WikiError::from(io_error);
    assert_eq!(wiki_error.description, "Not found");
}
