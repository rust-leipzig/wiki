//! # wiki
#![deny(missing_docs)]

extern crate markdown;
extern crate glob;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate mowl;

#[macro_use]
pub mod error;
pub mod processing;

use std::path::Path;
use std::env;
use clap::Arg;
use processing::Processing;
use error::{ErrorType, WikiResult};
use std::process::exit;

static ARG_INPUT_DIRECTORY: &'static str = "input-directory";
static ARG_OUTPUT_DIRECTORY: &'static str = "output-directory";
static DEFAULT_HTML_DIR: &'static str = "output";
static DEFAULT_MD_DEV_TEST_DIR: &'static str = "/tests/files";
static DEFAULT_MD_DIR: &'static str = "/files";

fn main() {
    if let Err(error) = run() {
        error!("{}", error);
        exit(1);
    }
}

fn run() -> WikiResult<()> {
    // Parse the given arguments
    let matches = app_from_crate!()
        .arg(Arg::from_usage("-o --output-directory=[PATH] 'The directory where the HTML output \
                              is generated.'"))
        //.arg(Arg::from_usage("<INPUT>                      'The directory containing the \
        //                      markdown files to use.'"))
        .arg(Arg::from_usage("-i --input-directory=[PATH]  'The directory containing the \
                        markdown files to use.'"))
        .get_matches();

    // Init logger crate
    match mowl::init() {
        Ok(_) => debug!("Mowl logging initiated."),
        Err(_) => {
            bail!(ErrorType::LoggerError,
                  "Initialization of mowl logger failed.")
        }
    }

    //info!("{}",matches.value_of(ARG_INPUT_DIRECTORY).unwrap_or("").to_string());

    let (abs_path_md_default, def_md_path_exists) =
        get_absolute_path_of_test_files(matches.value_of(ARG_INPUT_DIRECTORY)
            .unwrap_or("")
            .to_string());

    // true -> the path to the md folder exists
    // false -> the folder does not exists -> skip processing
    if def_md_path_exists == true {

        let md_dir = matches.value_of(ARG_INPUT_DIRECTORY).unwrap_or(&abs_path_md_default);
        let html_dir = matches.value_of(ARG_OUTPUT_DIRECTORY).unwrap_or(DEFAULT_HTML_DIR);



        // This can be deleted when html_dir is used further
        debug!("Output path: {}", html_dir);

        // Do first processing steps
        let mut processing = Processing::default();

        processing.read_from_directory(md_dir)?;
        processing.list_current_paths();
        processing.read_content_from_current_paths()?;
    } else {
        error!("Neither input-directory (-i) or the default markdown folders exists.");
    }

    Ok(())
}

fn get_absolute_path_of_test_files(input_value: String) -> (String, bool) {

    if input_value != "" {

        debug!("found parameter: {}",&input_value);

        // if the input-directory parameter is not empty, prefer this
        let path_exists = Path::new(&input_value).exists();

        (input_value, path_exists)
    }
    // if no path is given by the the "-i" Parameter, we sould try some fallback folder
    else {
        // fallback folder 1:    ./wiki|wiki.exe/files

        // get path of the executable
        let mut p = env::current_exe().unwrap();

        // remove "/wiki" frpm path
        p.pop();

        debug!("path of executable: {:?}", &p);

        // concatenate development folder root with relative path to the test md files
        let p_abs = format!("{}{}", p.to_string_lossy(), DEFAULT_MD_DIR);

        if Path::new(&p_abs).exists() {
            debug!("use fallback folder 1: {}", &p_abs);
            return (p_abs, true);

        }

        // fallback folder 2: developmentfolder "/tests/files""  (the executable is placed in the folder "/target/debug/files"")
        // this means, go 2 folder up and join "/tests/files "

        p.pop(); // remove "/debug"
        p.pop(); // remove "/target"

        // concatenate development folder root with relative path to the test md files
        let p_abs = format!("{}{}", p.to_string_lossy(), DEFAULT_MD_DEV_TEST_DIR);

        if Path::new(&p_abs).exists() {
            debug!("use fallback folder 2: {}", &p_abs);
            return (p_abs, true);

        }

        debug!("nothing found: {}", &p_abs);
        // no parmeter and no default folders found -> nothing to processing
        return ("".to_string(), false);


    }
}