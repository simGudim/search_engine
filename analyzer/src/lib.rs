mod sorting;
mod readers;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use porter_stemmer::stem;
use std::collections::{HashSet, HashMap};
use stopwords::{Spark, Language, Stopwords};


#[derive(Debug)]
pub struct WordStats {
    pub docs: HashSet<i32>,
    pub position: Vec<i32>,
    pub word_length: i32,
    pub freq: i32,
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
        assert_eq!(readers::read_pdf("/Users/simon/Documents/code/git_repo/test_data/1st Advantage Federal Credit Union/Mastercard Consumer Agreement.pdf"), "foo".to_owned());
    }

    #[test]
    fn read_text() { 
        assert_eq!(readers::read_text("/Users/simon/Downloads/test_data/abstracts/Aafaq_Spatio-Temporal_Dynamics_and_Semantic_Attribute_Enriched_Visual_Encoding_for_Video_CVPR_2019_paper.txt"), "foo".to_owned())
    }

}
