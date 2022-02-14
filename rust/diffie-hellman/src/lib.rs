fn primes(limit: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = (0..=limit)
        .map(|num| num == 2 || num & 1 != 0)
        .collect();

    let mut num = 3usize;
    while num * num < limit {
        let mut j = num * num;
        while j <= limit {
            primes[j] = false;
            j += num;
        }
        num +=2;
    }

    primes
        .into_iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, p)|
            if p {
                Some(i)
            } else {
                None
            })
        .collect::<Vec<usize>>()
}

pub fn private_key(p: u64) -> u64 {
    unimplemented!("Pick a private key greater than 1 and less than {}", p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    unimplemented!(
        "Calculate public key using prime numbers {} and {}, and private key {}",
        p,
        g,
        a
    )
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    unimplemented!(
        "Calculate secret key using prime number {}, public key {}, and private key {}",
        p,
        b_pub,
        a
    )
}
