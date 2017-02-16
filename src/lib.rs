//! The lib for markdown based static HTML wiki generation

#[macro_use]
extern crate log;
extern crate mowl;
extern crate glob;
extern crate markdown;

use glob::glob;
use markdown::to_html;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::str;

#[macro_use]
mod error;
pub use error::{ErrorType, WikiResult};

#[derive(Default)]
/// Global processing structure
pub struct Wiki {
    /// A collection of paths for the processing
    paths: Vec<PathBuf>,
}

impl Wiki {
    /// Create a new `Wiki` instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new instance of the processing lib
    pub fn init_logging(&mut self) -> WikiResult<()> {
        // Init logger crate
        match mowl::init() {
            Ok(_) => debug!("Mowl logging initiated."),
            Err(_) => {
                bail!(ErrorType::LoggerError,
                      "Initialization of mowl logger failed.")
            }
        }

        Ok(())
    }

    /// Reads all markdown files recursively from a given directory.
    /// Clears the current available paths
    pub fn read_from_directory(&mut self, directory: &str) -> WikiResult<()> {
        /// Remove all paths
        self.paths.clear();

        /// Gather new content
        let md_path = PathBuf::from(&directory).join("**").join("*.md");
        if !Path::new(&directory).is_dir() {
            bail!(ErrorType::PathNotExisting,
                  "The path '{}' does not exist", directory);
        }

        /// Use the current working directory as a fallback
        for entry in glob(md_path.to_str().unwrap_or("."))? {
            self.paths.push(entry?);
        }

        Ok(())
    }

    /// Print absolute path of all added md files
    pub fn list_current_paths(&self) {
        info!("Found the following markdown files:");
        for file in &self.paths {
            println!("    - {:?}", file);
        }
    }

    /// Read the content of all files and convert it to HTML
    pub fn read_content_from_current_paths(&self) -> WikiResult<()> {
        // Iterate over all available paths
        for file in &self.paths {
            info!("Parsing file: {}", file.display());

            // Open the file and read its content
            let mut f = File::open(file)?;
            let mut buffer = String::new();
            f.read_to_string(&mut buffer)?;
            debug!("{}", to_html(&buffer));
        }

        Ok(())
    }
}
