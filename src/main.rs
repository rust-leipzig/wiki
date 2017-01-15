//! # wiki
//!
#![deny(missing_docs)]

extern crate markdown;

use std::str;
use std::fs::File;
use std::io::prelude::*;

#[derive(Default)]
struct MdFiles {
    rel_path_of_md_files: Vec<String>,
}

impl MdFiles {
    /// Add an absolute path of an markdown file to the struct
    /// absolute path = working directory + inputparameter (relative path in the project directory)
    fn add_new_md_file(&mut self, rel_path: String) {
        let pwd = std::env::current_dir().unwrap();

        let mut path = std::path::PathBuf::new();
        path.push(pwd.to_str().unwrap());
        path.push(rel_path);

        self.rel_path_of_md_files.push(path.to_str().unwrap().to_string());
    }

    /// print absolute path of all added md files
    fn print_all_md_paths(&self) {
        for file in self.rel_path_of_md_files.iter() {
            println!("{:?}", file);
        }

    }

    /// read content of all files and convert it to html
    fn read_content_from_md_file(&self) {
        for md_file in self.rel_path_of_md_files.iter() {
            println!("parse file: {}", md_file);

            /// Error handling is not necessary here because github issue #1 "Collect all markdown files from a certain directory" should ensure that the opened file definitive exist.
            /// But it is better to have error handling :)
            let res = File::open(md_file);
            match res {
                Ok(mut file) => {

                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).unwrap();
                    let md_str = str::from_utf8(&buffer).unwrap();
                    println!("{}", markdown::to_html(md_str));

                }
                Err(e) => {
                    println!("{}: {}", md_file, e);
                }
            }

        }
    }
}

fn main() {
    let mut md = MdFiles::default();

    // need a data structure with all MD files: if Issue #1 is solved, 'add_new_md_file' could be deleted
    md.add_new_md_file("README.md".to_string());
    //md.add_new_md_file("examples/gitignore.md".to_string()); // does not exist

    md.print_all_md_paths();
    md.read_content_from_md_file();
}
