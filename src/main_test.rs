#[cfg(test)]

use std::process::Command;

#[test]
fn test_run_mowl_init_ok() {
    match Command::new("./target/debug/wiki").arg("./test/example_md/real_md").output() {
        Ok(output) => assert!(output.status.success()),
        Err(_) => panic!("Failed to get return value."),
    }
}

#[test]
fn test_run_mowl_init_fails() {
    match Command::new("./target/debug/wiki").output() {
        Ok(output) => assert!(!output.status.success()),
        Err(_) => panic!("Failed to get return value."),
    }
}
