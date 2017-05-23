//! Everything related to wikis filehash functionality

use InputPaths;
use error::*;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{Write, Read, BufReader, BufRead};
use sha_1::{Sha1, Digest};

pub struct Filehash {
}

impl Filehash {
    /// Reads the file hash for the file specified by `file_str` out of `hash_file_str`
    fn read_file_hash(hash_file_str: &str, file_str: &str) -> Option<(String)> {
        let hash_file_res = File::open(hash_file_str);
        if hash_file_res.is_err() {
            return None;
        }
        let hash_file_reader = BufReader::new(hash_file_res.unwrap());
        for line in hash_file_reader.lines() {
            match line {
                Ok(l) => {

                    // Break the line between `<hash>:<file>`
                    let sha_args: Vec<&str> = l.split(':').collect();

                    // File matched
                    if sha_args[1] == file_str {
                        return Some(String::from(sha_args[0]));
                    }
                },
                Err(_) => {},
            }
        }
        None
    }

    /// Writes all input files and their hashes into the file `hash_file_str`
    pub fn write_file_hash(input_paths: &mut Vec<InputPaths>, hash_file_str: &str) -> Result<()> {
        // Renew the hash_file
        if Path::new(hash_file_str).exists() {
            fs::remove_file(hash_file_str)?;
        }
        let mut hash_file = File::create(hash_file_str)?;

        // Write content into file in form `<hash>:<file>`
        let mut hash_file_content = String::new();
        for input_path in input_paths {
            hash_file_content.push_str(format!("{}:{}\n",
                                               input_path.hash.as_str(),
                                               input_path.path.to_str()
                                               .ok_or_else(|| "Unable to stringfy input path.")?)
                                       .as_str());
        }
        hash_file.write_all(hash_file_content.as_bytes())?;

        Ok (())
    }

    /// Calculate the hash of the given `file_str`
    fn get_file_hash(file_str: &str) -> Result<(String)> {
        let mut sha1 = Sha1::default();
        let mut buffer = String::new();
        let mut file_instance = File::open(PathBuf::from(file_str))?;

        file_instance.read_to_string(&mut buffer)?;

        sha1.input(buffer.as_bytes());
        let file_hash = sha1.result();

        let mut hash_str = String::new();
        for hash_byte in file_hash {
            hash_str.push_str(format!("{:x}", hash_byte).as_str());
        }
        debug!("Calculated file hash: {}", hash_str);

        Ok(hash_str)
    }

    /// Checks whether the calculated hash of `file_str` is equal to the hash stored
    /// in the file `hash_file_str`
    pub fn check_hash_currency(hash_file_str: &str, file_str: &str) -> Result<String> {
        debug!("Check hash currency of '{}'", file_str);
        let current_file_hash = Filehash::get_file_hash(file_str)?;
        match Filehash::read_file_hash(hash_file_str, file_str) {
            Some(stored_file_hash) => {
                // Stored file hash was found
                debug!("Extracted file hash:  {}", stored_file_hash);

                // Calculated hash of current file equals stored hash?
                if current_file_hash != stored_file_hash {
                    return Err(Error::from(current_file_hash));
                } else {
                    return Ok(current_file_hash);
                }
            },
            None => {
                // No stored hash found for this file
                return Err(Error::from(current_file_hash));
            },
        }
    }
}
