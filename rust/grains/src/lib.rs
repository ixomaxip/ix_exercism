pub fn square(s: u32) -> u64 {
    let base: u64 = 2;
    match s {
        1..=64 => base.pow(s - 1),
        _ => panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    (square(64) - 1) * 2 + 1
}
