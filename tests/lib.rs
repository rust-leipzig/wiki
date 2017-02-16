extern crate wikilib;
use wikilib::{Wiki, ErrorType};

#[test]
fn test_read_from_directory() {
    let mut wiki = Wiki::new();
    assert!(wiki.init_logging().is_ok());
    assert!(wiki.read_from_directory("tests/example_md/real_md").is_ok());
    assert!(wiki.read_content_from_current_paths().is_ok());
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

