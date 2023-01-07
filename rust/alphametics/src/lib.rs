#[warn(unused_imports)]
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn is_valid(words: &Vec<&str>, mapping: &HashMap<&char, &&u8>) -> bool {
    for w in words {
        let fst = w.chars().nth(0).unwrap();
        if mapping[&fst] == &&0 {
            return false;
        }
    }
    true
}

fn parse_word(word: &str, mapping: &HashMap<&char, &&u8>) -> i32 {
    let mut number: i32 = 0;
    let l = word.len();
    for (idx, c) in word.chars().enumerate() {
        number += **mapping[&c] as i32 * i32::pow(10, (l - idx - 1) as u32);
    }
    number

}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let sep = Regex::new(r".\+.|.==.");
    let words: Vec<_> = sep
        .unwrap()
        .split(input)
        .into_iter()
        .collect();
    println!("words: {:?}", words);
    let letters: Vec<char> = words.join("").chars().unique().sorted().collect();
    
    // let letters = input
    //     .chars()
    //     .filter(|c| c.is_alphanumeric())
    //     .fold(HashMap::new(), | mut map, c| {
        //         *map.entry(c).or_insert(0);
        //         map
        //     });
        
    println!("letters {:?}", letters.len());
    let digits: Vec<u8> = (0..=9).collect();
    for perm in digits.iter().permutations(letters.len()).unique() {
        // println!("{:?}", perm)
        let mapping: HashMap<&char, &&u8> = letters.iter().zip(perm.iter()).collect();
        if !is_valid(&words, &mapping)  {
            continue;
        }
        let mut sum: i32 = 0;
        for (w_idx, w) in words.iter().enumerate() {
            if w_idx == words.len() - 1 {
                sum += parse_word(w, &mapping);
            } else {
                sum -= parse_word(w, &mapping);
            }
        }
        
        if sum == 0 {
            println!("solution {:?}", mapping);
        }
    }
}
