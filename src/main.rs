
use std::str;
use std::vec::Vec;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


struct MD_FILES {
    rel_path_of_md_files: Vec<String>
}

impl MD_FILES {
    fn add_new_md_file(&mut self, rel_path: String) {

        let pwd = std::env::current_dir().unwrap();

        let mut path = std::path::PathBuf::new();
        path.push(pwd.to_str().unwrap());
        path.push(rel_path);

        self.rel_path_of_md_files.push(path.to_str().unwrap().to_string());
    }

    fn print_all_md_paths(&self) {
        for file in self.rel_path_of_md_files.iter() {
            println!("{:?}", file);
        }

    }

    fn read_content_from_md_file(&self) {
        
        for md_file in self.rel_path_of_md_files.iter() {
            println!("parse file: {}", md_file);

            let f = File::open(md_file).unwrap();
            let file = BufReader::new(&f);
            for (num, line) in file.lines().enumerate() {
                let l = line.unwrap();
                let n = num;
                println!("{}>{}", n, l);
            }

        }
    }
}

fn main() {

    let mut md = MD_FILES { rel_path_of_md_files: Vec::new() };

    // need a data structure with all MD files: if Issue #1 is solved, 'add_new_md_file' could be deleted
    md.add_new_md_file("README.md".to_string());
    // md.add_new_md_file("examples/gitignore.md".to_string()); // does not exist
    
    md.print_all_md_paths();
    md.read_content_from_md_file();
}
