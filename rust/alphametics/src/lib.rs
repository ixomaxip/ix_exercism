// #[warn(unused_imports)]
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn parse_word(word: &str, mapping: &HashMap<&char, &&i64>) -> i64 {
    let mut number = 0;
    let l = word.len();
    for (idx, c) in word.chars().enumerate() {
        number += **mapping[&c] * i64::pow(10, (l - idx - 1) as u32);
    }
    number
}

fn parse_words(words: &Vec<&str>, mapping: &HashMap<&char, &&i64>) -> Option<i64> {
    let mut sum = 0;
    for (w_idx, w) in words.iter().enumerate() {
        if **mapping[&w.chars().nth(0).unwrap()] == 0 {
            return None;
        }
        if w_idx == words.len() - 1 {
            sum += parse_word(w, mapping);
        } else {
            sum -= parse_word(w, mapping);
        }
    }
    Some(sum)
}

fn convert_solution(mapping: &HashMap<&char, &&i64>) -> HashMap<char, u8> {
    let mut result = vec![];
    for (key, val) in mapping {
        result.push((**key, ***val as u8));    
    }
    result.iter().cloned().collect()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let sep = Regex::new(r".\+.|.==.");
    let words: Vec<_> = sep
        .unwrap()
        .split(input)
        .into_iter()
        .collect();

    let letters: Vec<char> = words
        .join("")
        .chars()
        .unique()
        .sorted()
        .collect();
    
    let digits: Vec<i64> = (0..=9).collect();
    for perm in digits.iter().permutations(letters.len()).unique() {
        let mapping: HashMap<&char, &&i64> = letters.iter().zip(perm.iter()).collect();
        match parse_words(&words, & mapping) {
            Some(_s) if _s == 0 => return Some(convert_solution(&mapping)),
            Some(_s) => continue,
            None => continue
        }
    }
    None
}
