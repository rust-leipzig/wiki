//! The executable wiki for wikilib
#![deny(missing_docs)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate wikilib;

use clap::Arg;
use std::process::exit;
use wikilib::Processing;
use wikilib::error::WikiResult;

static ARG_INPUT_DIRECTORY: &'static str = "INPUT";
static ARG_OUTPUT_DIRECTORY: &'static str = "output-directory";
static DEFAULT_HTML_DIR: &'static str = "output";

fn main() {
    if let Err(error) = run() {
        error!("{}", error);
        exit(1);
    }
}

fn run() -> WikiResult<()> {
    // Parse the given arguments
    let matches = app_from_crate!()
        .arg(Arg::from_usage("-o --output-directory=[PATH] 'The directory where the HTML output is generated.'"))
        .arg(Arg::from_usage("<INPUT>                      'The directory containing the markdown files to use.'"))
        .get_matches();

    let md_dir = matches.value_of(ARG_INPUT_DIRECTORY).unwrap();
    let html_dir = matches.value_of(ARG_OUTPUT_DIRECTORY).unwrap_or(DEFAULT_HTML_DIR);

    // This can be deleted when html_dir is used further
    debug!("Output path: {}", html_dir);

    // Do first processing steps
    let mut processing = Processing::default();

    processing.init_logging()?;
    processing.read_from_directory(md_dir)?;
    processing.list_current_paths();
    processing.read_content_from_current_paths()?;

    Ok(())
}

