pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let result = (1..limit)
        .filter(|x| factors
            .iter()
            .any(|y| match y {
                0 => false,
                _ => x % y == 0
            }))
        .collect::<Vec<_>>();
    result.iter().sum()
}