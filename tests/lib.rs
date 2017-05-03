extern crate log;
extern crate wikilib;
extern crate glob;

use log::LogLevel;
use wikilib::Wiki;

use std::path::Path;
use std::fs;

static NON_EXISTING_DIR: &str = "_should_not_exist_";
static TMP_DIR: &str = "_tmp_dir_";

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
    println!("The following paths were found:");
    wiki.list_current_paths();
}

#[test]
fn test_read_from_non_existing_directory() {
    let mut wiki = Wiki::new();
    if wiki.read_from_directory("_non-exisiting_").is_ok() {
        panic!("`read_from_directory` returned ok, but directory should not exist.");
    }
}

#[test]
fn test_read_non_existing_content() {
    let mut wiki = Wiki::new();
    if Path::new(TMP_DIR).exists() {
        assert!(fs::remove_dir_all(TMP_DIR).is_ok());
    }
    assert!(fs::create_dir(TMP_DIR).is_ok());
    assert!(fs::File::create(Path::new(TMP_DIR).join("test.md")).is_ok());
    assert!(wiki.read_from_directory(TMP_DIR).is_ok());
    assert!(fs::remove_dir_all(TMP_DIR).is_ok());
    match wiki.read_content_from_current_paths(TMP_DIR, NON_EXISTING_DIR) {
        Ok(_) => panic!("`read_content_from_current_paths` returned ok, but should fail."),
        Err(_) => assert!(fs::remove_dir_all(NON_EXISTING_DIR).is_ok())
    }
}
