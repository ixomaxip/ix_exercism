pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![];

    let mut nn = n;
    while nn % 2 == 0 {
        primes.push(2);
        nn /= 2;
    }
    let upper = (nn as f64).sqrt() as u64 + 1;
    for i in (3..=upper).step_by(2) {
        while nn % i == 0 {
            primes.push(i);
            nn /= i;
        }
    }
    if nn > 2 { primes.push(nn)}
    primes
}
