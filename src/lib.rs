//! The lib for markdown based static HTML wiki generation

#[macro_use]
extern crate log;
extern crate glob;
extern crate iron;
extern crate markdown;
extern crate mowl;

use glob::glob;
use log::LogLevel;
use markdown::to_html;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;

use std::fs::{self, canonicalize, create_dir_all, File};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::io::prelude::*;
use std::str;

#[macro_use]
mod error;
pub use error::{ErrorType, WikiError, WikiResult};

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
    pub fn init_logging(&mut self, level: LogLevel) -> WikiResult<()> {
        // Init logger crate
        match mowl::init_with_level(level) {
            Ok(_) => info!("Log level set to: {}", level),
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
                  "The path '{}' does not exist",
                  directory);
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
    pub fn read_content_from_current_paths(&self, input_root_dir: &str,
                                           output_directory: &str) -> WikiResult<()> {
        // Check whether output_directory exists, if not -> create
        if !Path::new(output_directory).exists() {
            info!("Creating directory for HMTL output: '{}'.", output_directory);
            fs::create_dir(output_directory)?;
        }

        // Iterate over all available paths
        for file in &self.paths {
            info!("Parsing file: {}", file.display());

            // Open the file and read its content
            let mut f = File::open(file)?;
            let mut buffer = String::new();
            f.read_to_string(&mut buffer)?;

            // Creating the related HTML file in output_directory
            match file.to_str() {
                Some(file_str) => {
                    // Get canonical normal forms of the input path and the recursively searched
                    // directories
                    let file_buf_n = canonicalize(&PathBuf::from(file_str))?;
                    let file_str_n = file_buf_n.to_str()
                                     .ok_or_else(|| {
                                         WikiError::new(ErrorType::Other,
                                         "Unable to stringify canonical normal form of md-file.")
                                     })?;
                    let input_root_buf_n = canonicalize(&PathBuf::from(input_root_dir))?;
                    let mut input_root_str_n = String::from(
                        input_root_buf_n.to_str()
                        .ok_or_else(|| {
                            WikiError::new(ErrorType::Other,
                            "Unable to stringify canonical normal form of input root.")
                        })?
                    );

                    // Add native seperator to avoid getting the wrong path
                    input_root_str_n.push(MAIN_SEPARATOR);

                    // Reduce the input dir and replace the extension
                    let output_str = String::from(file_str_n)
                        .replace(input_root_str_n.as_str(), "")
                        .replace(".md", ".html");
                    let output_path = Path::new(output_str.as_str());

                    match output_path.parent() {
                        Some(parent) => {
                            // Creating folder structure if neccessary
                            let parent_path = Path::new(output_directory)
                                .join(parent.to_str().unwrap_or("."));
                            create_dir_all(parent_path)?;
                        },
                        None => {
                            bail!(ErrorType::Other,
                                  "Can't get output path parent.");
                        },
                    }
                    // Creating the ouput HTML file
                    let mut of = File::create(
                        PathBuf::from(&output_directory)
                            .join(output_path))?;
                    of.write(to_html(&buffer).as_bytes())?;
                },
                None => {
                    bail!(ErrorType::Other,
                          "Can not stringfy file path");
                }
            }
        }

        Ok(())
    }

    /// Create an HTTP server serving the generated files
    pub fn serve(&self, output_directory: &str) -> WikiResult<()> {
        // Create a default listening address
        let addr = "localhost:5000";
        info!("Listening on {}", addr);

        // Moving the data into the closure
        let output_directory_string = output_directory.to_owned();

        // Create a new iron instance
        Iron::new(move |request: &mut Request| {
                // The owned path needs to created from the cloned string
                let mut path = PathBuf::from(output_directory_string.clone());

                // Create the full path
                for part in request.url.path() {
                    path.push(part);
                }

                /* Could use some security validation for the path here. */

                // Use a default page for the middleware
                if path.is_dir() {
                    path.push("index.html");
                }

                if !path.exists() {
                    return Ok(Response::with(status::InternalServerError));
                }

                let mut f = match File::open(path) {
                    Ok(v) => v,
                    _ => return Ok(Response::with(status::NotFound)),
                };

                let mut buffer = String::new();
                match f.read_to_string(&mut buffer) {
                    Ok(v) => v,
                    _ => return Ok(Response::with(status::InternalServerError)),
                };

                /* Content type needs to be determined from the file rather than assuming html */
                Ok(Response::with((ContentType::html().0, status::Ok, buffer)))

            }).http(addr)?;

        Ok(())
    }
}
