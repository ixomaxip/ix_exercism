use std::{collections::HashMap, str::from_utf8, thread};
// use std::sync::Mutex;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    
    let input = input
        .join("")
        .to_lowercase();

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
        let result = dbg!(t.join().unwrap());
        for (&char, &count) in result.iter() {
            *final_result.entry(char).or_insert(0) += count;
        }
    }

    return final_result;
}

fn worker(text: &str) -> HashMap<char, usize> {
    let result = text
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::new(), |mut hmap, c| {
            *hmap.entry(c).or_insert(0) += 1;
            hmap
        });
    return result
}
