use rand::Rng;

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
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

fn mod_pow(p: u64, g: u64, a: u64) -> u64 {    
    let mut x = g as u128;
    let mut y = a as u128;
    let big_p = p as u128;

    x = x % p as u128;

    let mut res: u128 = 1;
    while y > 0 {
        if y & 1 != 0 {
            res = (res * x) % big_p;
        }
        y = y >> 1;
        x = (x * x) % big_p;
    }
    res as u64
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(p, g, a)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(p, b_pub, a)
}
