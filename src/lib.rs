 //! The lib for markdown based static HTML wiki generation

#[macro_use]
extern crate log;
extern crate glob;
extern crate iron;
extern crate markdown;
extern crate mowl;
#[macro_use]
extern crate error_chain;

pub mod error;

use error::*;
use glob::glob;
use log::LogLevel;
use markdown::to_html;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::mime::Mime;


use std::fs::{self, canonicalize, create_dir_all, File, OpenOptions};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::io::prelude::*;
use std::str;

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
    pub fn init_logging(&mut self, level: LogLevel) -> Result<()> {
        // Init logger crate
        match mowl::init_with_level(level) {
            Ok(_) => info!("Log level set to: {}", level),
            Err(_) => bail!("Initialization of mowl logger failed."),
        }

        Ok(())
    }
    /// Reads all markdown files recursively from a given directory.
    /// Clears the current available paths
    pub fn read_from_directory(&mut self, directory: &str) -> Result<()> {
        /// Remove all paths
        self.paths.clear();
        /// Gather new content
        let md_path = PathBuf::from(&directory).join("**").join("*.md");
        if !Path::new(&directory).is_dir () {
            bail!("The path '{}' does not exist", directory);
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
                                           output_directory: &str) -> Result<()> {
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
                    // Get canonical normal forms of the input path and the recursively
                    // searched directories
                    let file_buf_n = canonicalize(&PathBuf::from(file_str))?;
                    let file_str_n = file_buf_n.to_str()
                                     .ok_or_else(|| "Unable to stringify canonical normal form of md-file.")?;
                    let input_root_buf_n = canonicalize(&PathBuf::from(input_root_dir))?;
                    let mut input_root_str_n = String::from(
                        input_root_buf_n.to_str()
                        .ok_or_else(|| "Unable to stringify canonical normal form of input root.")?
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
                        None => bail!("Can't get output path parent."),
                    }

                    // Creating the ouput HTML file
                    let mut of = File::create(
                        PathBuf::from(&output_directory)
                            .join(output_path))?;
                    of.write(to_html(&buffer).as_bytes())?;
                },
                None => bail!("Can not stringfy file path"),
            }
        }

        Ok(())
    }
    /// read all the files from current paths
    pub fn read_files(&self, file_directory: &str) {
        //create the default file path
        if !Path::new(file_directory).exists(){
            info!("Creating directory for file input: {}.", file_directory);
            fs::create_dir(file_directory);
        }
        for path in self.paths.clone(){
            let mut count = 0;

            //create the path for the html site or just find it
            let page_files_path = format!("{}/{}", file_directory, path.to_str().unwrap()).replace(".md","");
            if !Path::new(page_files_path.as_str()).exists(){
                info!("Creating directory for {}'s files: {}.", path.to_str().unwrap(), page_files_path.as_str());
                fs::create_dir_all(page_files_path.as_str()).unwrap();
            }

            //add the path
            let file_path = Path::new(page_files_path.as_str());
            //read all the files
            let files = file_path.read_dir().unwrap();

            //now attach the files to the html sites folder
            for entry in files {
                let current_entry = match entry {
                    Ok(entry) => entry,
                    _ => panic!("Couldn't read the path!"),
                };

                let link = format!(
                    "\n<a href='http://localhost:3000/files/{}/{}'>{}</a><br>\n",
                    path.to_str().unwrap().replace(".md", ""),
                    current_entry.file_name().into_string().unwrap().replace("\n","."),
                    current_entry.file_name().into_string().unwrap()
                );

                let mut html_file = OpenOptions::new().read(true).write(true).create(true).
                    open(format!("output/{}", path.to_str().unwrap().
                    replace(".md", ".html"))).unwrap();

                let mut buffer = String::new();

                html_file.read_to_string(&mut buffer);
                buffer += link.clone().as_str();
                info!("Creating link to {:?} for {:?}", current_entry.file_name(), path.to_str().unwrap().replace(".md", ".html"));
                html_file.write((&buffer).as_bytes());
                count += 1;
            }
            //check if there are no files attached
            if count == 0 {
                info!("No files found so far for {}. Simply add your files to the folders in 'files'.", path.to_str().unwrap())
            }
        }
    }

    /// Create an HTTP server serving the generated files
    pub fn serve(&self, output_directory: &str) -> Result<()> {
        // Create a default listening address
        let addr = "localhost:3000";
        info!("Listening on {}", addr);

        // Moving the data into the closure
        let output_directory_string = output_directory.to_owned();

        //maybe this shoul be stored in a separated file?
        let pdf_mime: Mime = "application/pdf".parse().unwrap();
        let doc_mime: Mime = "application/msword".parse().unwrap();
        let oda_mime: Mime = "application/oda".parse().unwrap();
        let zip_mime: Mime = "application/zip".parse().unwrap();
        let wav_mime: Mime = "audio/wav".parse().unwrap();
        let jpg_mime: Mime = "image/jpeg".parse().unwrap();
        let png_mime: Mime = "image/png".parse().unwrap();
        let gif_mime: Mime = "image/gif".parse().unwrap();
        let css_mime: Mime = "text/css".parse().unwrap();
        let mpg_mime: Mime = "video/x-msvideo".parse().unwrap();
        let avi_mime: Mime = "video/avi".parse().unwrap();

        // Create a new iron instance
        Iron::new(move |request: &mut Request| {
                ///to load files in browser
                fn get_file(mime_type: Mime, body: File) -> iron::Response {
                    let mut resp = Response::with((status::Ok, body));
                    resp.headers.set(ContentType(mime_type));
                    resp
                }
                // The owned path needs to be created from the cloned string
                let mut path = PathBuf::from(output_directory_string.clone());
                let html_err = "<h1 id='not_found_error'>Error: 404 File not found</h1>".to_string();
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
                    return Ok(Response::with((ContentType::html().0,
                    status::NotFound, html_err)));
                }
                let mut f = match File::open(path.clone()) {
                    Ok(v) => v,
                    _ => return Ok(Response::with((ContentType::html().0,
                                status::NotFound, html_err))),
                    };

                match path.clone().to_str(){
                    Some(name) => {

                        if name.contains(".pdf") {return Ok(get_file(pdf_mime.clone(), f))};
                        if name.contains(".doc") {return Ok(get_file(doc_mime.clone(), f))};
                        if name.contains(".oda") {return Ok(get_file(oda_mime.clone(), f))};
                        if name.contains(".zip") {return Ok(get_file(zip_mime.clone(), f))};
                        if name.contains(".wav") {return Ok(get_file(wav_mime.clone(), f))};
                        if name.contains(".css") {return Ok(get_file(css_mime.clone(), f))};
                        if name.contains(".mp")  {return Ok(get_file(mpg_mime.clone(), f))};
                        if name.contains(".avi") {return Ok(get_file(avi_mime.clone(), f))};
                        if name.contains(".png") {return Ok(get_file(png_mime.clone(), f))};
                        if name.contains(".jp")  {return Ok(get_file(jpg_mime.clone(), f))};
                        if name.contains(".gif") {return Ok(get_file(gif_mime.clone(), f))};

                        if name.contains(".html") {
                            let mut buffer = String::new();
                            match f.read_to_string(&mut buffer) {
                                Ok(v) => v,
                                _ => return Ok(Response::with(status::InternalServerError)),
                            };
                            return Ok(Response::with((ContentType::html().0, status::Ok, buffer)))
                        }

                        else {Ok(Response::with((status::Ok, f)))}
                    },
                    _ => panic!("Invalid Path."),
                }
            }).http(addr)?;
        Ok(())
    }
}
