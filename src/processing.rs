//! Everything related to the markdown processing
use glob::glob;
use markdown::to_html;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::str;

use error::{WikiError, ErrorType};

#[derive(Default)]
/// Global processing structure
pub struct Processing {
    /// A collection of paths for the processing
    paths: Vec<PathBuf>,
}

/// The global result type for wiki
pub struct WikiResult {
    /// The `WikiError` error type for the current result
    pub error: WikiError
}

impl Processing {
    /// Reads all markdown files recursively from a given directory
    /// Clears the current available paths
    pub fn read_from_directory(&mut self, directory: &str) -> WikiError {
        /// Remove all paths
        self.paths.clear();

        /// Gather new content
        let md_path = PathBuf::from(&directory).join("**").join("*.md");
        if Path::new(&directory).is_dir() == false {
            return WikiError::new(ErrorType::PathNotExisting, &format!("The path '{}' does not exist", directory));
        }

        match glob(md_path.to_str().unwrap()) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(path) => self.paths.push(path),
                        Err(e) => return WikiError::new(ErrorType::PathNotReadable, &e.to_string()),
                    }
                }
            },
            Err(e) => return WikiError::new(ErrorType::FileNotReadable, e.msg),
        }
        WikiError::new(ErrorType::Ok, "")
    }

    /// Read the content of all files and convert it to HTML
    pub fn read_content_from_current_paths(&self) -> WikiError {
        // Iterate over all available paths
        for file in &self.paths {
            info!("Parsing file: {}", file.display());

            // Open the file and read its content
            match File::open(file) {
                Ok(mut file) => {
                    let mut buffer = String::new();
                    match file.read_to_string(&mut buffer) {
                        Ok(_) => debug!("{}", to_html(&buffer)),
                        Err(e) => return WikiError::new(ErrorType::BufferStringifyFailed, &e.to_string()),
                    }
                }
                Err(e) => return WikiError::new(ErrorType::FileNotReadable , &e.to_string()),
            }
        }
        WikiError::new(ErrorType::Ok, "")
    }

    /// Print absolute path of all added md files
    pub fn list_current_paths(&self) {
        info!("Found the following markdown files:");
        for file in &self.paths {
            println!("    - {:?}", file);
        }
    }
}
