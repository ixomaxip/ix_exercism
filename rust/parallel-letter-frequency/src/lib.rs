use std::{collections::HashMap,
    str::from_utf8,
    thread
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    let input = input
        .join("");

    let chunk_len = match input.len() / worker_count {
        0 => input.len() + 1,
        l => l + 1,
    };

    let chunked_data = input
        .as_bytes()
        .chunks(chunk_len)
        .map(|chunk| match from_utf8(chunk) {
            Ok(c) => c.to_string(),
            _ => "".to_string(),
        });

    let mut threads = vec![];
    for chunk in chunked_data {
        threads.push(thread::spawn(move || -> HashMap<char, usize> {
            worker(&chunk)
        }));
    }

    let mut final_result = HashMap::new();
    for t in threads {
        let result = t.join().unwrap();
        for (&char, &count) in result.iter() {
            *final_result.entry(char).or_insert(0) += count;
        }
    }

    return final_result;
}

fn worker(text: &String) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for chr in text.chars().filter(|c| c.is_alphabetic()) {
        if let Some(c) = chr.to_lowercase().next() {
            (*map.entry(c).or_insert(0)) += 1;
        }
    }
    map
}