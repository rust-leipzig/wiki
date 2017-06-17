//! The executable wiki for wikilib
#![deny(missing_docs)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate glob;
extern crate iron;
extern crate wikilib;
#[macro_use]
extern crate error_chain;

pub mod error;

use wikilib::Wiki;
use wikilib::error::*;

use clap::App;
use log::LogLevel;

use std::process::exit;

fn main() {
    if let Err(error) = run() {
        error!("{}", error);
        exit(1);
    }
}

// The main running function
fn run() -> Result<()> {
    // Parse the given arguments
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!());
    let matches = app.get_matches();

    // Set the verbosity level
    let log_level = match matches.occurrences_of("verbose") {
        0 => LogLevel::Info, // Default value
        1 => LogLevel::Debug,
        _ => LogLevel::Trace,
    };

    // Get the input directory
    let input_directory = matches.value_of("input_directory")
        .ok_or_else(|| "CLI parameter 'input_directory' missing.")?;

    // Get the output directory
    let output_directory = matches.value_of("output_directory")
        .ok_or_else(|| "CLI parameter 'output_directory' missing.")?;

    let enable_httpd = matches.is_present("www");

    let file_directory = matches.value_of("file_directory")
        .ok_or_else(|| "CLI parameter 'file_directory' missing")?;

    // Do first processing steps
    let mut wiki = Wiki::new();

    wiki.init_logging(log_level)?;
    wiki.read_from_directory(input_directory)?;
    wiki.read_content_from_current_paths(input_directory, output_directory)?;
    wiki.read_files(file_directory, output_directory);
    wiki.create_index_tree(output_directory)?;

    if enable_httpd {
        wiki.serve(output_directory)?;
    }

    Ok(())
}
