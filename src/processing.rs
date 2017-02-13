//! Everything related to the markdown processing

use glob::glob;
use markdown::to_html;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::str;

use error::{ErrorType, WikiResult};

#[derive(Default)]
/// Global processing structure
pub struct Processing {
    /// A collection of paths for the processing
    paths: Vec<PathBuf>,
}

impl Processing {
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

        /// Leave it as unwrap for now because of unimplemented Carrier trait
        for entry in glob(md_path.to_str().unwrap())? {
            let path = (entry)?;
            self.paths.push(path);
        }

        Ok(())
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

    /// Print absolute path of all added md files
    pub fn list_current_paths(&self) {
        info!("Found the following markdown files:");
        for file in &self.paths {
            println!("    - {:?}", file);
        }
    }
}
