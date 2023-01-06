#[warn(unused_imports)]
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

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
        

    Some(HashMap::new())
}
