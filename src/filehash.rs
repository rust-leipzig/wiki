//! Everything related to wikis filehash functionality

use error::*;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use uuid::{Uuid, NAMESPACE_DNS};
use InputPaths;

pub struct Filehash;

impl Filehash {
    /// Reads the file hash for the file specified by `file_str` out of `hash_file_str`
    fn read_file_hash(hash_file_str: &str, file_str: &str) -> Option<String> {
        if let Ok(hash_file_res) = File::open(hash_file_str) {
            let hash_file_reader = BufReader::new(hash_file_res);
            for line in hash_file_reader.lines() {
                if let Ok(l) = line {
                    // Break the line between `<hash>:<file>`
                    let sha_args: Vec<&str> = l.split(':').collect();

                    // File matched
                    if let Some(sha_arg_file) = sha_args.get(1) {
                        if sha_arg_file != &file_str {
                            continue;
                        }
                        if let Some(sha_arg_hash) = sha_args.get(0) {
                            return Some(String::from(*sha_arg_hash));
                        }
                    }
                }
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
            hash_file_content.push_str(
                format!(
                    "{}:{}\n",
                    input_path.hash.as_str(),
                    input_path
                        .path
                        .to_str()
                        .ok_or_else(|| "Unable to stringfy input path.")?
                )
                .as_str(),
            );
        }
        hash_file.write_all(hash_file_content.as_bytes())?;

        Ok(())
    }

    /// Calculate the hash of the given `file_str`
    fn get_file_hash(file_str: &str) -> Result<String> {
        let mut buffer = String::new();
        let mut file_instance = File::open(file_str)?;

        file_instance.read_to_string(&mut buffer)?;

        let file_uuid = Uuid::new_v5(&NAMESPACE_DNS, buffer.as_str());
        debug!("Calculated file hash: {}", file_uuid);

        Ok(file_uuid.to_string())
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
                    Err(Error::from(current_file_hash))
                } else {
                    Ok(current_file_hash)
                }
            }
            None => {
                // No stored hash found for this file
                Err(Error::from(current_file_hash))
            }
        }
    }
}
