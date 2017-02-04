//! # wiki
#![deny(missing_docs)]

extern crate markdown;
extern crate glob;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate mowl;

#[macro_use]
pub mod error;
pub mod processing;

use clap::Arg;
use processing::Processing;
use error::{WikiError, ErrorType};
use std::process::exit;

static ARG_INPUT_DIRECTORY: &'static str = "INPUT";
static ARG_OUTPUT_DIRECTORY: &'static str = "output-directory";
static DEFAULT_HTML_DIR: &'static str = "output";

fn error_and_exit(error: Box<WikiError>) {
    error!("{}", error);
    exit(1);
}

fn run() -> Result<(), Box<WikiError>> {
    // Parse the given arguments
    let matches = app_from_crate!()
        .arg(Arg::from_usage("-o --output-directory=[PATH] 'The directory where the HTML output is generated.'"))
        .arg(Arg::from_usage("<INPUT>                      'The directory containing the markdown files to use.'"))
        .get_matches();

    let md_dir = matches.value_of(ARG_INPUT_DIRECTORY).unwrap();
    let html_dir = matches.value_of(ARG_OUTPUT_DIRECTORY).unwrap_or(DEFAULT_HTML_DIR);

    // Init logger crate
    match mowl::init() {
        Ok(_) => debug!("Mowl logging initiated."),
        Err(_) => return Err(Box::new(WikiError::new(ErrorType::LoggerError, "Initialization of mowl logger failed."))),
    }

    // This can be deleted when html_dir is used further
    debug!("Output path: {}", html_dir);

    // Do first processing steps
    let mut processing = Processing::default();

    try!(processing.read_from_directory(&md_dir));
    processing.list_current_paths();
    try!(processing.read_content_from_current_paths());
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {},
        Err(retval) => error_and_exit(retval),
    }
}
