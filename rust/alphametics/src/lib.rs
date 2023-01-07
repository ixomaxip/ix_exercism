#[warn(unused_imports)]
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn is_valid(words: &Vec<&str>, mapping: &HashMap<&char, &&i32>) -> bool {
    for w in words {
        let fst = w.chars().nth(0).unwrap();
        if **mapping[&fst] == 0 {
            return false;
        }
    }
    true
}

fn parse_word(word: &str, mapping: &HashMap<&char, &&i32>) -> i32 {
    let mut number = 0;
    let l = word.len();
    for (idx, c) in word.chars().enumerate() {
        number += **mapping[&c] * i32::pow(10, (l - idx - 1) as u32);
    }
    number
}

fn convert_solution(mapping: &HashMap<&char, &&i32>) -> HashMap<char, u8> {
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
    
    let digits: Vec<i32> = (0..=9).collect();
    for perm in digits.iter().permutations(letters.len()).unique() {
        let mapping: HashMap<&char, &&i32> = letters.iter().zip(perm.iter()).collect();
        if !is_valid(&words, &mapping)  {
            continue;
        }
        let mut sum = 0;
        for (w_idx, w) in words.iter().enumerate() {
            if w_idx == words.len() - 1 {
                sum += parse_word(w, &mapping);
            } else {
                sum -= parse_word(w, &mapping);
            }
        }
        if sum == 0 {
            let solution = convert_solution(&mapping);
            return Some(solution);
        }
    }
    None
}
