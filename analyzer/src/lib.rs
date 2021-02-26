mod sorting;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use porter_stemmer::stem;
use std::collections::{HashSet, HashMap};
use stopwords::{Spark, Language, Stopwords};
use bson::Bson;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::path::PathBuf;
use std::path;
use std::io::BufWriter;
use pdf_extract::*;
use lopdf::*;
use std::str;



#[derive(Debug)]
pub struct WordStats {
    pub docs: HashSet<i32>,
    pub position: Vec<Vec<Bson>>,
    pub word_length: i32,
    pub freq: i32,
}


pub fn create_tokens_list<'a>(text: &'a String) -> Vec<String> {
    let re = Regex::new("[^0-9a-zA-Z]+").unwrap();
    let text_no_special_char = re.replace_all(text.as_str(), " ");
    let tokenised_sentence = text_no_special_char.unicode_words();
    let tokens: String = tokenised_sentence.map(stem).fold(String::new(), |last, next| { format!("{}{} ", &last, &next)});
    //need to move this into Struct
    let stops: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
    let mut word_list: Vec<&str> = tokens.split(" ").collect();
    word_list.retain(|s| !stops.contains(s));
    (*word_list).to_vec().iter().map(|x| x.to_string().to_lowercase()).collect::<Vec<String>>()
}

pub fn create_index(data: Vec<Vec<String>>) -> HashMap<String, WordStats> {
    let mut index: HashMap<String, WordStats> = HashMap::new();
    for i in 0..data.len() {
        for x in 0..data[i].len() {
            if index.contains_key(&data[i][x]) {
                (*index.get_mut(&data[i][x]).unwrap()).docs.insert(i as i32);
                let docs_len = (*index.get_mut(&data[i][x]).unwrap()).docs.len();
                let pos_len = (*index.get_mut(&data[i][x]).unwrap()).position.len();
                if docs_len > pos_len {
                    let temp = vec![Bson::from(x as i32)];
                    (*index.get_mut(&data[i][x]).unwrap()).position.push(temp);
                } else {
                    (*index.get_mut(&data[i][x]).unwrap()).position[pos_len - 1].push(Bson::from(x as i32));
                }
                (*index.get_mut(&data[i][x]).unwrap()).freq += 1;
            } else {
                let mut docs: HashSet<i32> = HashSet::new();
                docs.insert(i as i32);

                let mut position: Vec<Vec<Bson>> = vec![vec![]];
                position[0].push(Bson::from(x as i32));

                let word_length = data[i][x].chars().count() as i32;

                let stat = WordStats {
                    docs,
                    position,
                    word_length,
                    freq: 1
                };
                index.insert(data[i][x].clone(), stat);
            }
        }
    }
    index
}

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
    // let path = path::Path::new(&path);
    // let file = File::<Vec<u8>>::open(&path).unwrap();
    // let sparkle_heart = str::from_utf8(&file).unwrap();
    // println!("{}", sparkle_heart);
    "foo".to_owned()
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn execute_test() {
        assert_eq!(1+1 , 2);
    }

    #[test]
    #[ignore]
    fn test_mergesort() {
        let mut items: Vec<&str> = vec!["ann", "black", "shoe", "tree", "jack", "abb"];
        let check: Vec<&str> = vec!["abb", "ann", "black", "jack", "shoe", "tree"];
        items = sorting::mergesort(items);
        assert_eq!(items, check);
    }

    #[test]
    #[ignore]
    fn test_readpdf() {
        assert_eq!(read_pdf("/Users/simon/Documents/code/git_repo/test_data/1st Advantage Federal Credit Union/Mastercard Consumer Agreement.pdf"), "foo".to_owned());
    }

}
