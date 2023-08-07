use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn parse_word(word: &str, mapping: &HashMap<char, u8>) -> u64 {
    let mut number = 0;
    let mut mult = u64::pow(10, (word.len() - 1) as u32);
    for c in word.chars() {
        number += mapping[&c] as u64 * mult;
        mult /= 10;
    }
    number
}

fn parse_words(words: &Vec<&str>, mapping: &HashMap<char, u8>) -> Option<bool> {
    let mut lhs_sum = 0;
    let mut rhs = 0;
    for (w_idx, w) in words.iter().enumerate() {
        if mapping[&w.chars().nth(0).unwrap()] == 0 {
            return None;
        }
        if w_idx == words.len() - 1 {
            lhs_sum += parse_word(w, mapping);
        } else {
            rhs += parse_word(w, mapping);
        }
    }
    match lhs_sum == rhs {
        true => Some(true),
        false => None
    }
    // Some(sum)
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

    for perm in (0..=9).permutations(letters.len()) {
        let mapping = HashMap::from_iter(letters.iter().map(|x| *x as char).zip(perm.iter().map(|x| *x as u8)));
        match parse_words(&words, &mapping) {
            Some(_)=> return Some(mapping),
            None => continue
        }
    }
    None
}
