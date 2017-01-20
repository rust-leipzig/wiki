//! Everything related to the markdown processing
use glob::glob;
use markdown::to_html;

use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::str;

#[derive(Default)]
/// Global processing structure
pub struct Processing {
    /// A collection of paths for the processing
    paths: Vec<PathBuf>,
}

impl Processing {
    /// Reads all markdown files recursively from a given directory
    /// Clears the current available paths
    pub fn read_from_directory(&mut self, directory: &str) {
        /// Remove all paths
        self.paths.clear();

        /// Gather new content
        let md_path = PathBuf::from(&directory).join("**/*.md");

        /// TODO: Error handling
        for entry in glob(md_path.to_str().unwrap()).expect("Failed to read glob pattern.") {
            match entry {
                Ok(path) => self.paths.push(path),
                Err(e) => panic!(e.to_string()),
            }
        }
    }

    /// Read the content of all files and convert it to HTML
    pub fn read_content_from_current_paths(&self) {
        // Iterate over all available paths
        for file in &self.paths {
            println!("Parsing file: {}", file.display());

            // Open the file and read its content
            match File::open(file) {
                Ok(mut file) => {
                    let mut buffer = String::new();
                    // TODO: Error handling
                    file.read_to_string(&mut buffer).unwrap();
                    println!("{}", to_html(&buffer));
                }
                Err(e) => println!("{}: {}", file.display(), e),
            }
        }
    }

    /// Print absolute path of all added md files
    pub fn list_current_paths(&self) {
        for file in &self.paths {
            println!("{:?}", file);
        }
    }
}
