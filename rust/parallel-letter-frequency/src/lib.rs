use std::collections::HashMap;
use std::sync::Mutex;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.join("")
        .to_lowercase()
        .replace(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..], "")
        .replace(&['.', ',', '!', '?', ';', ':', ')', '(', '\'', '\"'][..], "");

    let result = input
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
    return result;
}

fn worker(data: &str) -> HashMap<char, usize> {
    data
        .to_lowercase()
        .replace(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..], "")
        .replace(&['.', ',', '!', '?', ';', ':', ')', '(', '\'', '\"'][..], "")
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}
