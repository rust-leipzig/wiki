//! # wiki
//!
#![deny(missing_docs)]

extern crate markdown;

extern crate getopts;
use getopts::Options;
use getopts::Matches;

extern crate glob;
use glob::glob;

use std::env;
use std::path::Path;

use std::str;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;

#[derive(Default)]
struct MdFiles {
    md_file_paths: Vec<String>
}

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

impl MdFiles {
    /// Add an absolute path of an markdown file to the struct
    /// absolute path = working directory + inputparameter (relative path in the project directory)
    /*fn add_new_md_file(&mut self, rel_path: String) {
        let pwd = std::env::current_dir().unwrap();

        let mut path = std::path::PathBuf::new();
        path.push(pwd.to_str().unwrap());
        path.push(rel_path);

        self.md_file_paths.push(path.to_str().unwrap().to_string());
    }*/

    /// Reads all markdown files recursively from a given directory
    fn get_markdown_from_dir(&mut self, md_dir: &str) {
        self.md_file_paths.clear();
        let md_path = Path::new(&md_dir).join("**").join("*.md");
        for entry in glob(&md_path.to_str().unwrap()).expect("Failed to read glob pattern.") {
            match entry {
                Ok(path) => self.md_file_paths.push(String::from_str(path.to_str().unwrap()).unwrap()),
                Err(e) => panic!(e.to_string()),
            }
        }
    }

    /// print absolute path of all added md files
    fn print_all_md_paths(&self) {
        for file in self.md_file_paths.iter() {
            println!("{:?}", file);
        }

    }

    /// read content of all files and convert it to html
    fn read_content_from_md_file(&self) {
        for md_file in self.md_file_paths.iter() {
            println!("parse file: {}", md_file);

            /// Error handling is not necessary here because github issue #1 "Collect all markdown files from a certain directory" should ensure that the opened file definitive exist.
            /// But it is better to have error handling :)
            let res = File::open(md_file);
            match res {
                Ok(mut file) => {

                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).unwrap();
                    let md_str = str::from_utf8(&buffer).unwrap();
                    println!("{}", markdown::to_html(md_str));

                }
                Err(e) => {
                    println!("{}: {}", md_file, e);
                }
            }

        }
    }
}

fn main() {
    let mut md = MdFiles::default();
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

    md.get_markdown_from_dir(&md_dir);
    md.print_all_md_paths();
    md.read_content_from_md_file();
}
