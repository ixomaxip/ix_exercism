use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn parse_word(word: &str, mapping: &HashMap<char, u8>) -> i64 {
    let mut number = 0;
    let l = word.len();
    for (idx, c) in word.chars().enumerate() {
        number += mapping[&c] as i64 * i64::pow(10, (l - idx - 1) as u32);
    }
    number
}

fn parse_words(words: &Vec<&str>, mapping: &HashMap<char, u8>) -> Option<i64> {
    let mut sum = 0;
    for (w_idx, w) in words.iter().enumerate() {
        if mapping[&w.chars().nth(0).unwrap()] == 0 {
            return None;
        }
        if w_idx == words.len() - 1 {
            sum += parse_word(w, mapping) as i64;
        } else {
            sum -= parse_word(w, mapping) as i64;
        }
    }
    Some(sum)
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
            Some(_s) if _s == 0 => return Some(mapping),
            Some(_s) => continue,
            None => continue
        }
    }
    None
}
