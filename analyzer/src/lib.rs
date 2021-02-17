use std::io::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use porter_stemmer::stem;
use std::collections::{HashSet, HashMap};
use stopwords::{Spark, Language, Stopwords};
use std::fs::{self, DirEntry};
use std::path::Path;


// pub struct Analyzer {
//     max_docs: i32
// }

#[derive(Debug)]
pub struct WordStats {
    docs: HashSet<i32>,
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
    let tokenised_sentence = text_no_special_char.unicode_words();
    let tokens: String = tokenised_sentence.map(stem).fold(String::new(), |last, next| { format!("{}{} ", &last, &next)});
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
                (*index.get_mut(&data[i][x]).unwrap()).position.push(x as i32);
                (*index.get_mut(&data[i][x]).unwrap()).freq += 1;
            } else {
                let mut docs: HashSet<i32> = HashSet::new();
                docs.insert(i as i32);

                let mut position = vec![];
                position.push(x as i32);

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

pub fn merge<T: Copy + PartialOrd>(a: &Vec<T>, mut l1: usize, r1: usize, mut l2: usize, r2: usize,) -> Vec<T> {
    let mut temp = vec![];
    let mut index = 0;
    while l1 <= r1 && l2 <= r2 {
        if a[l1] <= a[l2] {
            temp.push(a[l1]);
            index += 1;
            l1 += 1
        } else {
            temp.push(a[l2]);
            index += 1;
            l2 += 1;
        }
    }

    while l1 <= r1 {
        temp.push(a[l1]);
        index += 1;
        l1 += 1;
    }

    while l2 <= r2 {
        temp.push(a[l2]);
        index += 1;
        l2 += 1;
    }
    temp
}

pub fn mergesort<T: Copy + PartialOrd>(mut items: Vec<T>) -> Vec<T> {
    let mut size: usize = 1;
    let n: usize = items.len();
    while size < n {
        let mut i: usize = 0;

        while i < n {
            let l1: usize = i;
            let r1: usize = i + size - 1;
            let mut r2: usize= i + 2 * size - 1;
            let l2: usize= i + size;

            if l2 >= n {
                break
            }

            if r2 >= n {
                r2 = n - 1;
            }

            let temp = merge(&items, l1, r1, l2, r2);
            for j in 0..(r2-l1 +1) {
                items[i + j] = temp[j];
            }
            i = i + 2 * size;
        }
        size = 2 * size;
    }
    items
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute() {
        let docs: Vec<String> = read_file_from_dir("./test_data");
        let mut temp = vec![];
        for i in docs {
            temp.push(create_tokens_list(&i));
        }
        println!("{:?}", create_index(temp));
        assert_eq!(1+1 , 2);
    }

    #[test]
    fn test_mergesort() {
        let mut items: Vec<&str> = vec!["ann", "black", "shoe", "tree", "jack", "abb"];
        let check: Vec<&str> = vec!["abb", "ann", "black", "jack", "shoe", "tree"];
        items = mergesort(items);
        assert_eq!(items, check);
    }
}
