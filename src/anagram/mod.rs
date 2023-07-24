use std::{collections::HashSet, hash::Hash};
use unicode_segmentation::UnicodeSegmentation;


// the first way
// pub fn anagrams_for<'a> (word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str>{
//     let word = word.to_lowercase();
//     let mut chars_vec: Vec<&str> = word.graphemes(true).collect();
//     chars_vec.sort_unstable();
//     let result = possible_anagrams.iter()
//         .enumerate().map(|(i,_)| {i})
//         .filter(|i|{
//             let tested_word = &possible_anagrams[*i];
//             if tested_word.len() != word.len() {
//                 return false;
//             }
//             let tested_word = tested_word.to_lowercase();
//             if tested_word == word {
//                 return false;
//             }
//             let mut tested_chars_vec: Vec<&str> = tested_word.graphemes(true).collect();
//             tested_chars_vec.sort_unstable();
//             tested_chars_vec == chars_vec
//             }
//         ).map(|i| {possible_anagrams[i]}).collect();
//         result
// }


// the second way
// fn get_sorted(word: &str) -> Vec<char> {
//     let mut word_sorted = word.chars().collect::<Vec<char>>();
//     word_sorted.sort_unstable();
//     word_sorted
// }

// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//     let word_lower = word.to_lowercase();
//     let word_sorted = get_sorted(&word_lower);
//     possible_anagrams
//         .iter()
//         .filter(|candidate| {
//             let candidate_lower = candidate.to_lowercase();
//             candidate_lower.len() == word_lower.len()
//                 && candidate_lower != word_lower
//                 && get_sorted(&candidate_lower) == word_sorted
//         })
//         .copied()
//         .collect()
// }

//the third way
fn case_norm(word: &str) -> String{
    word.to_lowercase()
}
fn norm(word: &str) -> String {
    let word = word.to_string();
    let mut chars = word.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn anagrams_for<'a> (word: &str, possible_anagram: &'a[&str]) -> HashSet<&'a str>{
    let word_case_norm = case_norm(word);
    let word_norm = norm(&word_case_norm);
    possible_anagram
        .iter()
        .filter(|candidate|{
            let candidate_case_norm = case_norm(candidate);
            let candidate_norm = norm(&candidate_case_norm);
            candidate_case_norm != word_case_norm && candidate_norm == word_norm
        })
        .copied()
        .collect()

}

// Test
fn process_anagram_case(word: &str, input: &[&str], expected: &[&str]){
    let result = anagrams_for(word, input);
    let expected: HashSet<&str> = expected.iter().cloned().collect();
    assert_eq!(result, expected);
}


fn test_no_matches() {
    let word = "diaper";
    let inputs = ["hello", "world", "zombies", "pants"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}

fn test_does_not_confuse_different_duplicates() {
    let word = "galea";
    let inputs = ["eagle"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}

fn test_eliminate_anagram_subsets() {
    let word = "good";
    let inputs = ["dog", "goody"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}

fn test_same_bytes_different_chars() {
    let word = "a⬂"; // 61 E2 AC 82
    let inputs = ["€a"]; // E2 82 AC 61
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}


pub fn test() {
    test_no_matches();
    test_does_not_confuse_different_duplicates();
    test_eliminate_anagram_subsets();
    test_same_bytes_different_chars();
    println!("it's ok");
}
