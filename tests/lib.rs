extern crate glob;
extern crate log;
extern crate wikilib;

use log::LevelFilter;
use wikilib::Wiki;

use std::fs;
use std::path::Path;

static NON_EXISTING_DIR: &str = "_should_not_exist_";
static TMP_DIR: &str = "_tmp_dir_";

#[test]
fn test_read_from_directory() {
    let mut wiki = Wiki::new();
    assert!(wiki.init_logging(LevelFilter::Trace).is_ok());
    let input_dir = "tests/example_md/real_md";
    assert!(wiki.read_from_directory(input_dir).is_ok());
    let sha_file = Path::new("html").join(".files.sha");
    if sha_file.exists() {
        assert!(fs::remove_file(&sha_file).is_ok());
    }
    assert!(wiki
        .read_content_from_current_paths(input_dir, "html")
        .is_ok());
    assert!(sha_file.exists());
    assert!(Path::new("html").exists());
    println!("The following paths were found:");
    wiki.list_current_input_paths();
    let index_file = Path::new("html").join("index.html");
    if index_file.exists() {
        assert!(fs::remove_file(&index_file).is_ok());
    }
    assert!(wiki.create_index_tree("html").is_ok());
    let check_paths = vec![
        "html/index.html",
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
fn test_sha_file_existing() {
    let mut wiki = Wiki::new();
    let input_dir = "tests/example_md/real_md";
    assert!(wiki.read_from_directory(input_dir).is_ok());
    assert!(wiki
        .read_content_from_current_paths(input_dir, "html2")
        .is_ok());
    let sha_file = Path::new("html2").join(".files.sha");
    assert!(sha_file.exists());
    assert!(wiki
        .read_content_from_current_paths(input_dir, "html2")
        .is_ok());
}

#[test]
fn test_read_from_non_existing_directory() {
    let mut wiki = Wiki::new();
    assert!(wiki.read_from_directory("_non-exisiting_").is_err())
}

#[test]
fn test_read_non_existing_content() {
    let mut wiki = Wiki::new();
    assert!(fs::create_dir(TMP_DIR).is_ok());
    assert!(fs::File::create(Path::new(TMP_DIR).join("test.md")).is_ok());
    assert!(wiki.read_from_directory(TMP_DIR).is_ok());
    assert!(fs::remove_dir_all(TMP_DIR).is_ok());
    match wiki.read_content_from_current_paths(TMP_DIR, NON_EXISTING_DIR) {
        Ok(_) => panic!("`read_content_from_current_paths` returned ok, but should fail."),
        Err(_) => assert!(!Path::new(NON_EXISTING_DIR).exists()),
    }
}
