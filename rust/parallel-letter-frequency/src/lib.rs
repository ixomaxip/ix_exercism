use std::collections::HashMap;
use std::str::from_utf8;
// use std::sync::Mutex;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.join("")
        .to_lowercase()
        .replace(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..], "")
        .replace(&['.', ',', '!', '?', ';', ':', ')', '(', '\'', '\"'][..], "");

    println!("FULL TEXT: {}", input.len());
    let chunk_len = match input.len() / worker_count {
        0 => 1,
        l => l + 1,
    };
    let chunked_data = input
        .as_bytes()
        .chunks(chunk_len)
        .map(|chunk| {
            match from_utf8(chunk) {
                Ok(c) => c,
                _ => ""

            }});
    // let mut treads = vec![];
    for (i, chunk) in chunked_data.enumerate() {
        println!("chunk: {}: {}", i, chunk.len());
        println!("{:?}", chunk);
    }

    let one_thread_result = input
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
    return one_thread_result;
}

// fn worker(data: &str) -> HashMap<char, usize> {
//     data
//         .to_lowercase()
//         .replace(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..], "")
//         .replace(&['.', ',', '!', '?', ';', ':', ')', '(', '\'', '\"'][..], "")
//         .chars()
//         .fold(HashMap::new(), |mut map, c| {
//             *map.entry(c).or_insert(0) += 1;
//             map
//         })
// }
