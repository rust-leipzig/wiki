 //! The lib for markdown based static HTML wiki generation

#[macro_use]
extern crate log;
extern crate glob;
extern crate iron;
extern crate markdown;
extern crate mowl;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

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

lazy_static! {
    static ref PDF_MIME: Mime = "application/pdf".parse().unwrap();
    static ref DOC_MIME: Mime = "application/msword".parse().unwrap();
    static ref ODA_MIME: Mime = "application/oda".parse().unwrap();
    static ref ZIP_MIME: Mime = "application/zip".parse().unwrap();
    static ref WAV_MIME: Mime = "application/wav".parse().unwrap();
    static ref CSS_MIME: Mime = "application/css".parse().unwrap();
    static ref GIF_MIME: Mime = "application/gif".parse().unwrap();
    static ref MPG_MIME: Mime = "application/x-msvideo".parse().unwrap();
    static ref AVI_MIME: Mime = "application/avi".parse().unwrap();
    static ref PNG_MIME: Mime = "application/png".parse().unwrap();
    static ref JPG_MIME: Mime = "application/jpeg".parse().unwrap();
}

#[derive(Default)]
/// Global processing structure
pub struct Wiki {
    /// A collection of paths for the processing
    paths: Vec<PathBuf>,
    file_paths: Vec<(PathBuf, PathBuf)>,
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
        if !Path::new(&directory).is_dir() {
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
    /// Add a directory for storing files to the generated html sites or read stored
    /// files of existing filestorage. Afterwards file links will be added to the
    /// generated html site.
    pub fn read_files(&mut self, file_folder: &str, output: &str) -> Result<()> {
        //create the default file path
        let output_path = PathBuf::from(output);
        let file_directory = PathBuf::from(output).join(file_folder);
        if !Path::new(file_folder).exists(){
            info!("Creating directory for file input: {:?}.", file_directory.to_str());
            fs::create_dir(&file_directory);
        }
        for path in &self.paths{
            let mut count = 0;

            //create the path for the html site or just find it
            let page_files_path = file_directory.join(path).to_str().ok_or_else(|| "Couldn't create Path to html file!")?
                                  .replace(".md","");
            if !PathBuf::from(page_files_path.as_str()).exists(){
                info!("Creating directory for {}'s files: {}.", path.to_str().ok_or_else(|| "Path not found!")?,
                                                                page_files_path.as_str());
                fs::create_dir_all(page_files_path.as_str());
            }

            //add the path
            let file_path = Path::new(page_files_path.as_str());
            //read all the files
            let files = file_path.read_dir()?;

            //now attach the files to the html sites folder
            for entry in files {
                let current_entry = match entry {
                    Ok(entry) => entry,
                    _ => panic!("Couldn't read the path!"),
                };

                let link = format!(
                    "\n<a href='http://localhost:30000/files/{}/{}'>{}</a><br>\n",
                                path.to_str().ok_or_else(|| "Path not found!")?.replace(".md", ""),
                    current_entry.file_name().to_str().ok_or_else(|| "Entry is corrupted!")?.replace("\n","."),
                    current_entry.file_name().to_str().ok_or_else(|| "Entry is corrupted!")?
                );
                self.file_paths.push(
                    (Path::new(current_entry.file_name().to_str()
                                .ok_or_else(|| "Entry is corrupted!")?).to_owned(),
                    Path::new(path.to_str()
                                .ok_or_else(|| "Entry is corrupted!")?.replace(".md", "")
                                .as_str()).to_owned())
                );
                let mut html_file = OpenOptions::new().read(true).write(true).create(true).
                    open(output_path.join(path).to_str().ok_or_else(|| "Entry is corrupted!")?.
                    replace(".md", ".html"))?;

                let mut buffer = String::new();

                html_file.read_to_string(&mut buffer);
                buffer += &link.as_str();
                info!("Creating link to {:?} for {:?}", current_entry.file_name(), path.to_str().
                                ok_or_else(|| "Entry is corrupted!")?.replace(".md", ".html"));
                html_file.write((&buffer).as_bytes());
                count += 1;
            }
            //check if there are no files attached
            if count == 0 {
                info!("No files found so far for {}. Simply add your
                    files to the folders in 'files'.",
                    path.to_str().ok_or_else(|| "Entry is corrupted!")?)
            }
        }
        Ok(())
    }

    /// Create an HTTP server serving the generated files
    pub fn serve(&self, output_directory: &str) -> Result<()> {
        // Create a default listening address
        let addr = "localhost:30000";
        info!("Listening on {}", addr);

        // Moving the data into the closure
        let output_directory_string = output_directory.to_owned();

        // Create a new iron instance
        Iron::new(move |request: &mut Request| {
                ///to load files in browser
                fn get_file(mime_type: &Mime, body: File) -> iron::Response {
                    let mut resp = Response::with((status::Ok, body));
                    resp.headers.set(ContentType(mime_type.to_owned()));
                    resp
                }
                let mut path = PathBuf::from(&output_directory_string);
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
                let mut f = match File::open(&path) {
                    Ok(v) => v,
                    _ => return Ok(Response::with((ContentType::html().0,
                                status::NotFound, html_err))),
                    };

                match path.to_str(){
                    Some(name) => {

                        if name.contains(".pdf") {return Ok(get_file(&(*PDF_MIME), f))};
                        if name.contains(".doc") {return Ok(get_file(&(*DOC_MIME), f))};
                        if name.contains(".oda") {return Ok(get_file(&(*ODA_MIME), f))};
                        if name.contains(".zip") {return Ok(get_file(&(*ZIP_MIME), f))};
                        if name.contains(".wav") {return Ok(get_file(&(*WAV_MIME), f))};
                        if name.contains(".css") {return Ok(get_file(&(*CSS_MIME), f))};
                        if name.contains(".mp")  {return Ok(get_file(&(*MPG_MIME), f))};
                        if name.contains(".avi") {return Ok(get_file(&(*AVI_MIME), f))};
                        if name.contains(".png") {return Ok(get_file(&(*PNG_MIME), f))};
                        if name.contains(".jp")  {return Ok(get_file(&(*JPG_MIME), f))};
                        if name.contains(".gif") {return Ok(get_file(&(*GIF_MIME), f))};

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
