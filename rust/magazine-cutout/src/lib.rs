// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut freqs: HashMap<&str, u32> = HashMap::new();
    for word in magazine {
        *freqs.entry(word).or_insert(0) += 1;
    }
    for word in note {
        match freqs.get_mut(word) {
            Some(0) | None => return false,
            Some(v) => *v -= 1,
        }
    }
    return true;
}
