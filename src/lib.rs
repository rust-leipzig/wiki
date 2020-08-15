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
extern crate rayon;
extern crate uuid;

pub mod error;
pub mod filehash;

use error::*;
use glob::glob;
use log::LevelFilter;
use markdown::to_html;

use iron::headers::ContentType;
use iron::mime::Mime;
use iron::prelude::*;
use iron::status;

use filehash::Filehash;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::fs::{self, canonicalize, create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::str;

pub struct InputPaths {
    path: PathBuf,
    hash: String,
}

impl InputPaths {
    fn new(path: &str) -> Self {
        InputPaths {
            path: PathBuf::from(path),
            hash: String::new(),
        }
    }

    fn parse_as_html(
        &mut self,
        input_root_dir: &str,
        output_dir: &str,
        sha_file: &str,
    ) -> Result<PathBuf> {
        // Open the file and read its content
        let mut f = File::open(&self.path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        // Creating the related HTML file in output_directory
        match self.path.to_str() {
            Some(file_str) => {
                // Get canonical normal forms of the input path and the recursively
                // searched directories
                let file_buf_n = canonicalize(&PathBuf::from(file_str))?;
                let file_str_n = file_buf_n
                    .to_str()
                    .ok_or_else(|| "Unable to stringify canonical normal form of md-file.")?;
                let input_root_buf_n = canonicalize(&PathBuf::from(input_root_dir))?;
                let mut input_root_str_n =
                    String::from(input_root_buf_n.to_str().ok_or_else(|| {
                        "Unable to stringify canonical normal form of input root."
                    })?);

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
                        let parent_path =
                            Path::new(output_dir).join(parent.to_str().unwrap_or("."));
                        create_dir_all(parent_path)?;
                    }
                    None => bail!("Can't get output path parent."),
                }

                match Filehash::check_hash_currency(sha_file, file_str) {
                    Ok(hash) => {
                        // File hash is up to date, no need to rebuild
                        self.hash = hash;
                        debug!("File '{}' hash up to date.", file_str);
                    }
                    Err(hash) => {
                        // Creating the ouput HTML file
                        self.hash = hash.to_string();
                        info!("Parsing file: {}", file_str);
                        let output_file_path = PathBuf::from(&output_dir).join(output_path);
                        let mut output_file = File::create(&output_file_path)?;
                        output_file.write_all(to_html(&buffer).as_bytes())?;
                    }
                }
                Ok(output_path.to_path_buf())
            }
            None => bail!("Can not stringfy file path"),
        }
    }
}

lazy_static! {
    static ref PDF_MIME: Mime = "application/pdf".parse::<Mime>().unwrap();
    static ref DOC_MIME: Mime = "application/msword".parse::<Mime>().unwrap();
    static ref ODA_MIME: Mime = "application/oda".parse::<Mime>().unwrap();
    static ref ZIP_MIME: Mime = "application/zip".parse::<Mime>().unwrap();
    static ref WAV_MIME: Mime = "audio/x-wav".parse::<Mime>().unwrap();
    static ref CSS_MIME: Mime = "text/css".parse::<Mime>().unwrap();
    static ref GIF_MIME: Mime = "image/gif".parse::<Mime>().unwrap();
    static ref MPG_MIME: Mime = "video/mpeg".parse::<Mime>().unwrap();
    static ref AVI_MIME: Mime = "video/x-msvideo".parse::<Mime>().unwrap();
    static ref PNG_MIME: Mime = "image/png".parse::<Mime>().unwrap();
    static ref JPG_MIME: Mime = "image/jpeg".parse::<Mime>().unwrap();
    static ref SHA_FILE: &'static str = ".files.sha";
}

#[derive(Default)]
/// Global processing structure
pub struct Wiki {
    /// A collection of input_paths for the processing
    input_paths: Vec<InputPaths>,
    /// The html output paths
    output_paths: Vec<PathBuf>,
}

impl Wiki {
    /// Create a new `Wiki` instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new instance of the processing lib
    pub fn init_logging(&mut self, level: LevelFilter) -> Result<()> {
        // Init logger crate
        match mowl::init_with_level(level) {
            Ok(_) => info!("Log level set to: {}", level),
            Err(_) => bail!("Initialization of mowl logger failed."),
        }

        Ok(())
    }
    /// Reads all markdown files recursively from a given directory.
    /// Clears the current available input_paths
    pub fn read_from_directory(&mut self, directory: &str) -> Result<()> {
        // Remove all input_paths
        self.input_paths.clear();

        // Gather new content
        let md_path = PathBuf::from(&directory).join("**").join("*.md");
        if !Path::new(&directory).is_dir() {
            bail!("The path '{}' does not exist", directory);
        }

        // Use the current working directory as a fallback
        for entry in glob(md_path.to_str().unwrap_or("."))? {
            self.input_paths.push(InputPaths::new(
                entry?
                    .to_str()
                    .ok_or_else(|| "Unable to stringfy entry in markdown path.")?,
            ));
        }

        Ok(())
    }
    /// Print absolute path of all added md files
    pub fn list_current_input_paths(&self) {
        info!("Found the following markdown files:");
        for file in &self.input_paths {
            println!("    - {:?}", file.path);
        }
    }

    /// Read the content of all files and convert it to HTML
    pub fn read_content_from_current_paths(
        &mut self,
        input_root_dir: &str,
        output_directory: &str,
    ) -> Result<()> {
        if fs::read_dir(input_root_dir)?.count() == 0 {
            return Err(Error::from("No files given in input paths"));
        }

        // Check whether output_directory exists, if not -> create
        if !Path::new(output_directory).exists() {
            info!(
                "Creating directory for HMTL output: '{}'.",
                output_directory
            );
            fs::create_dir(output_directory)?;
        }

        let sha_file_path = PathBuf::from(output_directory).join(*SHA_FILE);
        let sha_file = sha_file_path
            .to_str()
            .ok_or_else(|| "Unable to stringify the sha file path.")?;

        // Iterate over all available input_paths
        self.output_paths = self
            .input_paths
            .par_iter_mut()
            .filter_map(|ref mut file| {
                file.parse_as_html(input_root_dir, output_directory, sha_file)
                    .ok()
            })
            .collect();

        Filehash::write_file_hash(&mut self.input_paths, sha_file)?;

        Ok(())
    }

    /// Creates an index.html with simple tree structure view when no index.md was seen
    pub fn create_index_tree(&self, output_directory: &str) -> Result<()> {
        let index_path = Path::new(output_directory).join("index.html");
        if !index_path.exists() {
            info!(
                "Creating index.html at {}",
                index_path
                    .to_str()
                    .ok_or_else(|| "Unable to stringify index path.")?
            );
            let mut index_file = File::create(index_path)?;
            let mut index_str = String::from(include_str!("html/index.template.html"));
            for output_path in &self.output_paths {
                index_str.push_str(
                    format!(
                        "<li><a href=\"{}\">{}</a></li>\n",
                        output_path
                            .to_str()
                            .ok_or_else(|| "Unable to stringify output path.")?,
                        output_path
                            .file_name()
                            .ok_or_else(|| "Unable to extract file name for path")?
                            .to_str()
                            .ok_or_else(|| "Unable to stringify output path.")?
                    )
                    .as_str(),
                );
            }
            index_file.write_all(index_str.as_bytes())?;
        }

        Ok(())
    }

    /// Add a directory for storing files to the generated html sites or read stored
    /// files of existing filestorage. Afterwards file links will be added to the
    /// generated html site.
    pub fn read_files(&mut self, file_folder: &str, output: &str) -> Result<()> {
        // create the default file path
        let output_path = PathBuf::from(output);
        let file_directory = PathBuf::from(output).join(file_folder);

        if !Path::new(file_folder).exists() {
            info!(
                "Creating directory for file input: {:?}.",
                file_directory.to_str()
            );
            fs::create_dir(&file_directory)?;
        }

        for path in &self.input_paths {
            let mut count = 0;

            // create the path for the html site or just find it
            let page_files_path = file_directory
                .join(&path.path)
                .to_str()
                .ok_or_else(|| "Couldn't create Path to html file!")?
                .replace(".md", "");

            if !PathBuf::from(page_files_path.as_str()).exists() {
                info!(
                    "Creating directory for {}'s files: {}.",
                    path.path.to_str().ok_or_else(|| "Path not found!")?,
                    page_files_path.as_str()
                );
                fs::create_dir_all(page_files_path.as_str())?;
            }

            // add the path
            let file_path = Path::new(page_files_path.as_str());
            //read all the files
            let files = file_path.read_dir()?;

            // now attach the files to the html sites folder
            for entry in files {
                let current_entry = match entry {
                    Ok(entry) => entry,
                    _ => panic!("Couldn't read the path!"),
                };

                let link = format!(
                    "\n<a href='http://localhost:30000/files/{}/{}'>{}</a><br>\n",
                    path.path
                        .to_str()
                        .ok_or_else(|| "Path not found!")?
                        .replace(".md", ""),
                    current_entry
                        .file_name()
                        .to_str()
                        .ok_or_else(|| "Entry is corrupted!")?
                        .replace("\n", "."),
                    current_entry
                        .file_name()
                        .to_str()
                        .ok_or_else(|| "Entry is corrupted!")?
                );
                let mut html_file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(
                        output_path
                            .join(&path.path)
                            .to_str()
                            .ok_or_else(|| "Entry is corrupted!")?
                            .replace(".md", ".html"),
                    )?;

                let mut buffer = String::new();

                html_file.read_to_string(&mut buffer)?;
                buffer += &link.as_str();
                info!(
                    "Creating link to {:?} for {:?}",
                    current_entry.file_name(),
                    path.path
                        .to_str()
                        .ok_or_else(|| "Entry is corrupted!")?
                        .replace(".md", ".html")
                );
                html_file.write_all((&buffer).as_bytes())?;
                count += 1;
            }
            // check if there are no files attached
            if count == 0 {
                info!("No files found so far for {}. Simply add your files to the folders in 'files'.",
                      path.path.to_str().ok_or_else(|| "Entry is corrupted!")?)
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
            // Create the full path
            for part in request.url.path() {
                path.push(part);
            }

            // Could use some security validation for the path here.

            // Use a default page for the middleware
            if path.is_dir() {
                path.push("index.html");
            }

            if !path.exists() {
                return Ok(Response::with((
                    ContentType::html().0,
                    status::NotFound,
                    include_str!("html/404.html"),
                )));
            }
            let mut f = match File::open(&path) {
                Ok(v) => v,
                _ => {
                    return Ok(Response::with((
                        ContentType::html().0,
                        status::NotFound,
                        include_str!("html/404.html"),
                    )))
                }
            };

            match path.to_str() {
                Some(name) => {
                    if name.contains(".pdf") {
                        return Ok(get_file(&(*PDF_MIME), f));
                    };
                    if name.contains(".doc") {
                        return Ok(get_file(&(*DOC_MIME), f));
                    };
                    if name.contains(".oda") {
                        return Ok(get_file(&(*ODA_MIME), f));
                    };
                    if name.contains(".zip") {
                        return Ok(get_file(&(*ZIP_MIME), f));
                    };
                    if name.contains(".wav") {
                        return Ok(get_file(&(*WAV_MIME), f));
                    };
                    if name.contains(".css") {
                        return Ok(get_file(&(*CSS_MIME), f));
                    };
                    if name.contains(".mp") {
                        return Ok(get_file(&(*MPG_MIME), f));
                    };
                    if name.contains(".avi") {
                        return Ok(get_file(&(*AVI_MIME), f));
                    };
                    if name.contains(".png") {
                        return Ok(get_file(&(*PNG_MIME), f));
                    };
                    if name.contains(".jp") {
                        return Ok(get_file(&(*JPG_MIME), f));
                    };
                    if name.contains(".gif") {
                        return Ok(get_file(&(*GIF_MIME), f));
                    };

                    if name.contains(".html") {
                        let mut buffer = String::new();
                        match f.read_to_string(&mut buffer) {
                            Ok(v) => v,
                            _ => {
                                return Ok(Response::with((
                                    ContentType::html().0,
                                    status::InternalServerError,
                                    include_str!("html/500.html"),
                                )))
                            }
                        };
                        Ok(Response::with((ContentType::html().0, status::Ok, buffer)))
                    } else {
                        Ok(Response::with((status::Ok, f)))
                    }
                }
                _ => panic!("Invalid Path."),
            }
        })
        .http(addr)?;
        Ok(())
    }
}
