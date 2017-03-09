extern crate log;
extern crate wikilib;
extern crate glob;

use log::LogLevel;
use wikilib::{Wiki, ErrorType};

use std::path::Path;
use glob::glob;

#[test]
fn test_read_from_directory() {
    let mut wiki = Wiki::new();
    assert!(wiki.init_logging(LogLevel::Trace).is_ok());
    assert!(wiki.read_from_directory("tests/example_md/real_md").is_ok());
    assert!(wiki.read_content_from_current_paths("html").is_ok());
    assert!(Path::new("html").exists());
    for entry in glob(Path::new("tests/example_md/real_md").join("*.md").to_str().unwrap()).unwrap() {
        match entry {
            Ok(path) => {
                let mut stem = String::from(path.file_stem().unwrap().to_str().unwrap());
                stem.push_str(".html");
                let file_name = Path::new(stem.as_str());
                let check_path = Path::new("html").join(file_name);
                assert!(check_path.exists());
            },
            Err(_) => panic!("`entry` read from markdown directory not ok."),
        }
    }
}

#[test]
#[should_panic]
fn test_read_from_directory_panic() {
    let mut wiki = Wiki::new();
    match wiki.read_from_directory("_non-exisiting_") {
        Ok(_) => return,
        Err(e) => {
            assert_eq!(e.code, ErrorType::PathNotExisting);
            panic!("`read_from_directory` returned ok, but directory should not exist.");
        },
    }
}

