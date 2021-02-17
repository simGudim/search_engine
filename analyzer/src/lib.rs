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


pub struct Analyzer {
    max_docs: i32
}

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
    let text_lowercase: &str = text_no_special_char.to_lowercase().as_ref();
    let tokenised_sentence = text_no_special_char.unicode_words();
    let tokens: String = tokenised_sentence.map(stem).fold(String::new(), |last, next| { format!("{}{} ", &last, &next)});
    let stops: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
    let mut word_list: Vec<&str> = tokens.split(" ").collect();
    word_list.retain(|s| !stops.contains(s));
    (*word_list).to_vec().iter().map(|x| x.to_string()).collect::<Vec<String>>()
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

pub fn merge(A: Vec<i32>, mut l1: usize, mut r1: usize, mut l2: usize, mut r2: usize,) -> Vec<i32> {
    let mut temp: Vec<i32> = vec![];
    let mut index = 0;
    while l1 <= r1 && l2 <= r2 {
        if A[l1] <= A[l2] {
            temp.push(A[l1]);
            index += 1;
            l1 += 1
        } else {
            temp.push(A[l2]);
            index += 1;
            l2 += 1;
        }
    }

    while l1 <= r1 {
        temp.push(A[l1]);
        index += 1;
        l1 += 1;
    }

    while l2 <= r2 {
        temp.push(A[l2]);
        index += 1;
        l2 += 1;
    }

    temp
}

pub fn mergesort(mut items: Vec<i32>) -> Vec<i32> {
    let mut size: usize = 1;
    let n: usize = items.len();
    while size < n {
        let mut i: usize = 0;

        while i < n {
            let mut l1: usize = i;
            let mut r1: usize = i + size - 1;
            let mut r2: usize= i + 2 * size - 1;
            let mut l2: usize= i + size;

            if l2 >= n {
                break
            }

            if r2 >= n {
                r2 = n - 1;
            }

            let temp = merge(items.clone(), l1, r1, l2, r2);
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
    #[ignore]
    fn general_test() {
        let docs: Vec<String> = read_file_from_dir("./test_data");
        let mut temp = vec![];
        for i in docs {
            temp.push(create_tokens_list(&i));
        }
        println!("{:?}", create_index(temp));
        assert_eq!(1+1 , 2);
    }

    #[test]
    fn test_sort() {
        let items: Vec<i32> = vec![2,4,7,5,9, 5, 10];
        let check: Vec<i32> = vec![2,4, 5, 5, 7, 9, 10];
        assert_eq!(mergesort(items), check);
    }
}
