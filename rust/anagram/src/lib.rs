use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let result: HashSet<&str> =
        possible_anagrams
            .iter().cloned()
            .filter(|&s| s.to_lowercase() != word.to_lowercase())
            .filter(|&s| s.len() == word.len())
            .filter(|&s| {
                            let mut word = word
                                .to_lowercase()
                                .chars()
                                .fold(HashMap::new(), |mut map, c| {
                                    *map.entry(c).or_insert(0) += 1;
                                    map
                                });
            
                            for c in s.to_lowercase().chars() {
                                word.entry(c).and_modify(|v| {*v -= 1});
                            }
                            word.values().all(|&x| x ==0)
                        }
                    )
            .collect();
    result
}
