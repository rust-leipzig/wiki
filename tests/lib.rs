extern crate log;
extern crate wikilib;
extern crate glob;

use log::LogLevel;
use wikilib::Wiki;

use std::path::Path;

#[test]
fn test_read_from_directory() {
    let mut wiki = Wiki::new();
    assert!(wiki.init_logging(LogLevel::Trace).is_ok());
    let input_dir = "tests/example_md/real_md";
    assert!(wiki.read_from_directory(input_dir).is_ok());
    assert!(wiki.read_content_from_current_paths(input_dir, "html").is_ok());
    assert!(Path::new("html").exists());
    let check_paths = vec![
        "html/code.html",
        "html/example1.html",
        "html/subsection/test_s1.html",
        "html/subsection/test_s2.html",
        "html/test1.html",
        "html/test2.html",
    ];
    for path in check_paths {
        assert!(Path::new(path).exists());
    }
}

#[test]
#[should_panic]
fn test_read_from_directory_panic() {
    let mut wiki = Wiki::new();
    match wiki.read_from_directory("_non-exisiting_") {
        Ok(_) => return,
        Err(_) => panic!("`read_from_directory` returned ok, but directory should not exist."),
    }
}

