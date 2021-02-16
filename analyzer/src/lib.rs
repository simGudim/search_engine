use std::io::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use porter_stemmer::stem;
use std::iter::FromIterator;
use std::collections::{HashSet, HashMap};
use stopwords::{Spark, Language, Stopwords};
use std::fs::{self, DirEntry};
use std::path::Path;


pub struct Analyzer {
    max_docs: i32
}

pub struct WordStats {
    docs: Vec<i32>,
    position: Vec<i32>,
    word_length: i32,
    freq: i32,
}


pub fn read_file_from_dir(path: &str) -> Vec<String> {
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

pub fn create_tokens_list<'a>(text: &'a String) -> Vec<String> {
    let re = Regex::new("[^0-9a-zA-Z]+").unwrap();
    let text_no_special_char = re.replace_all(text.as_str(), " ");
    let text_lowercase: &str = text_no_special_char.to_lowercase().as_ref();
    let tokenised_sentence = text_no_special_char.unicode_words();
    let tokens: String = tokenised_sentence.map(stem).fold(String::new(), |last, next| { format!("{}{} ", &last, &next)});
    let stops: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
    let mut word_list: Vec<&str> = tokens.split(" ").collect();
    word_list.retain(|s| !stops.contains(s));
    (*word_list).to_vec().iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

pub fn create_index(data: Vec<Vec<String>>) -> HashMap<String, WordStats> {
    let index = HashMap::new();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_read() {
        assert_eq!(read_file_from_dir("./test_data/article1.txt"), vec!["foo".to_owned()]);
    }

    #[test]
    #[ignore]
    fn test_split() {
        assert_eq!(create_tokens_list(&"foo,.,.,.,daksndaskdnk   ,dsadasd ,adasd, asdasd".to_owned()), vec!["foo".to_owned()]);
    }

    #[test]
    fn general_test() {
        let docs: Vec<String> = read_file_from_dir("./test_data");
        for i in docs {
            println!("{:?}", create_tokens_list(&i));
        }
        assert_eq!(create_tokens_list(&"dkasnd".to_owned()), vec!["foo".to_owned()]);
    }
}
