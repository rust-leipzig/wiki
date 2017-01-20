//! # wiki
//!
#![deny(missing_docs)]

extern crate markdown;
extern crate getopts;
extern crate glob;

pub mod processing;

use getopts::{Options, Matches};
use processing::Processing;
use std::env;

#[derive(Default)]
struct ArgOptions {
    args: Vec<String>,
    usage: String,
    matches: Option<Matches>
}

impl ArgOptions {
    /// Parses arguments, the matches and creates the usage briefing
    fn parse_args(&mut self) {
        self.args = env::args().collect();
        let mut opts = Options::new();

        opts.optopt("d", "dir", "Path to the markdown directory.", "PATH");
        opts.optflag("h", "help", "Print this help menu.");

        self.matches = match opts.parse(&self.args[1..]) {
            Ok(_match) => Some(_match),
            Err(e) => panic!(e.to_string()),
        };
        let brief = format!("Usage: {} FILE [options]", self.args[0]);
        self.usage = opts.usage(&brief);
    }
}

fn main() {
    // Parse the given arguments
    let mut argopts = ArgOptions::default();
    argopts.parse_args();

    let mut md_dir = String::new();
    if let Some(matches) = argopts.matches {
        if matches.opt_present("h") {
            print!("{}", argopts.usage);
            return;
        }
        match matches.opt_str("d") {
            Some(_match) => md_dir.push_str(&_match),
            None => {},
        }
    }

    // Do first processing steps
    let mut processing = Processing::default();
    processing.read_from_directory(&md_dir);
    processing.list_current_paths();
    processing.read_content_from_current_paths(); 
}
