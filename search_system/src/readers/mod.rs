use std::fs::{self, File};
use std::io::BufReader;
use std::io::BufWriter; 
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub fn read_files_from_dir(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let path_buf = Path::new(path);
    if path_buf.is_dir() {
        for entry in fs::read_dir(path_buf).unwrap() {
            let entry = entry.unwrap();
            let file = File::open(entry.path()).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            result.push(contents);
        }
    } else {
        panic!("the directory is not correct")
    }
    result
}

pub fn read_text(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

//################# Needs work###############
pub fn read_pdf(path: &str) -> String {
    "foo".to_owned()
}