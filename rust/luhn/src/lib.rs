fn calc(idx: usize, val: u32) -> u32 {
    match idx % 2 {
        0 => {
            let doubled = 2 * val;
            if doubled > 9 { doubled - 9 } else { doubled }
        },
        _ => val
    }
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let res: u32 = code
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(idx, val)| calc(idx, val))
        .sum();

    println!("{:?}", res);

    res % 10 == 0
}
