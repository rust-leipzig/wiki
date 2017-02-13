#[cfg(test)]

use processing::Processing;
#[allow(unused_imports)]
use error::ErrorType;

#[test]
fn test_read_from_directory() {
    let mut processing = Processing::default();
    match processing.read_from_directory("test/example_md/real_md") {
        Ok(_) => {},
        Err(e) => panic!(e.description),
    }
    match processing.read_content_from_current_paths() {
        Ok(_) => {},
        Err(e) => panic!("{}", e.to_string()),
    }
}

#[test]
#[should_panic]
fn test_read_from_directory_panic() {
    let mut processing = Processing::default();
    match processing.read_from_directory("_non-exisiting_") {
        Ok(_) => return,
        Err(e) => {
            assert_eq!(e.code, ErrorType::PathNotExisting);
            panic!("`read_from_directory` returned ok, but directory should not exist.");
        },
    }
}

