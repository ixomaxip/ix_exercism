#[warn(unused_imports)]
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn is_valid(words: &Vec<&str>, mapping: &HashMap<&char, &&u8>) -> bool {
    let mut result = true;
    for w in words {
        let fst = w.chars().nth(0).unwrap();
        if mapping[&fst] == &&0 {
            result = false;
        }
    }
    result
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
    let items: Vec<u8> = (0..9).collect();
    for perm in items.iter().permutations(letters.len()).unique() {
        // println!("{:?}", perm)
        let mapping: HashMap<&char, &&u8> = letters.iter().zip(perm.iter()).collect();
        println!("mapping: {:?}", mapping);
        if !is_valid(&words, &mapping)  {
            continue;
        }
    }    

    Some(HashMap::new())
}
