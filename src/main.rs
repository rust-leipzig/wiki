//! The executable wiki for wikilib
#![deny(missing_docs)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate wikilib;
extern crate iron;

use clap::Arg;
use std::process::exit;
use wikilib::{Wiki, WikiResult};

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use std::fs::metadata;

static ARG_INPUT_DIRECTORY: &'static str = "INPUT";
static ARG_OUTPUT_DIRECTORY: &'static str = "output-directory";
static DEFAULT_HTML_DIR: &'static str = "html";

fn main() {
    if let Err(error) = run() {
        error!("{}", error);
        exit(1);
    }
}

fn http_handler(req: &mut Request) -> IronResult<Response> {
    let mut path = PathBuf::new();

    path.push(DEFAULT_HTML_DIR); /* How to get the user option here? There seems to be no user data to pass it in. */

    for part in req.url.path() {
        path.push(part);
    }

    /* Could use some security validation for the path here. */

    if metadata(path.to_str().unwrap()).unwrap().is_dir() {
        path.push("index.html");
    }

    let path = match path.to_str() {
                   Some(v) => v,
                   None => return Ok(Response::with(status::InternalServerError))
               };

    let mut f = match File::open(path) {
                    Ok(v) => v,
                    Err(e) => return Ok(Response::with(status::NotFound)) /* Error type should be distinguished */
                };

    let mut buffer = String::new();
    let _ = match f.read_to_string(&mut buffer) {
                      Ok(v) => v,
                      Err(e) => return Ok(Response::with(status::InternalServerError))
                  };

    /* Content type needs to be determined from the file rather than assuming html */
    Ok(Response::with((ContentType::html().0, status::Ok, buffer)))
}

fn run() -> WikiResult<()> {
    // Parse the given arguments
    let matches = app_from_crate!()
        .arg(Arg::from_usage("-o --output-directory=[PATH] 'The directory where the HTML output is generated.'"))
        .arg(Arg::from_usage("-w --www                     'Enable integrated HTTP server to serve contents from output directory.'"))
        .arg(Arg::from_usage("<INPUT>                      'The directory containing the markdown files to use.'"))
        .get_matches();

    let md_dir = matches.value_of(ARG_INPUT_DIRECTORY).unwrap();
    let html_dir = matches.value_of(ARG_OUTPUT_DIRECTORY).unwrap_or(DEFAULT_HTML_DIR);
    let enable_httpd = matches.is_present("www");

    // Do first processing steps
    let mut wiki = Wiki::new();

    wiki.init_logging()?;
    wiki.read_from_directory(md_dir)?;
    wiki.list_current_paths();
    wiki.read_content_from_current_paths(html_dir)?;

    if enable_httpd {
        Iron::new(http_handler).http("localhost:5000").unwrap();
    }

    Ok(())
}

