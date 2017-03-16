//! The executable wiki for wikilib
#![deny(missing_docs)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate wikilib;

use wikilib::{ErrorType, Wiki, WikiError, WikiResult};

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
fn run() -> WikiResult<()> {
    // Parse the given arguments
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!());
    let matches = app.get_matches();

    // Set the verbosity level
    let log_level = match matches.occurrences_of("verbose") {
        0 => LogLevel::Error,
        1 => LogLevel::Warn,
        2 => LogLevel::Info,
        3 => LogLevel::Debug,
        _ => LogLevel::Trace,
    };

    // Get the input directory
    let input_directory = matches.value_of("input_directory")
        .ok_or_else(|| {
            WikiError::new(ErrorType::CliParameterMissing,
                           "Input directory CLI parameter missing")
        })?;

    // Get the output directory
    let output_directory = matches.value_of("output_directory")
        .ok_or_else(|| {
            WikiError::new(ErrorType::CliParameterMissing,
                           "Output directory CLI parameter missing")
        })?;

    let enable_httpd = matches.is_present("www");

    // Do first processing steps
    let mut wiki = Wiki::new();

    wiki.init_logging(log_level)?;
    wiki.read_from_directory(input_directory)?;
    wiki.list_current_paths();
    wiki.read_content_from_current_paths(input_directory, output_directory)?;

    if enable_httpd {
        wiki.serve(output_directory)?;
    }

    Ok(())
}
