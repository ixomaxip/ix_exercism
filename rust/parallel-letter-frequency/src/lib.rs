use std::collections::HashMap;
use std::sync::Mutex;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.join("");
    let result = worker(&input);
    return result;
}

fn worker(data: &str) -> HashMap<char, usize> {
    data
        .to_lowercase()
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}
