//! # wiki
//!
#![deny(missing_docs)]

extern crate markdown;
extern crate glob;
#[macro_use]
extern crate clap;

pub mod processing;

use clap::App;
use processing::Processing;

static ARG_INPUT_DIRECTORY: &'static str = "INPUT";
static ARG_OUTPUT_DIRECTORY: &'static str = "output-directory";
static DEFAULT_HTML_DIR: &'static str = "./out";

fn main() {
    // Parse the given arguments
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let md_dir = matches.value_of(ARG_INPUT_DIRECTORY).unwrap();
    let html_dir = matches.value_of(ARG_OUTPUT_DIRECTORY).unwrap_or(DEFAULT_HTML_DIR);

    // Do first processing steps
    let mut processing = Processing::default();
    processing.read_from_directory(&md_dir);
    processing.list_current_paths();
    processing.read_content_from_current_paths(); 
}
