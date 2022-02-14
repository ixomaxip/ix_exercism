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
    let mut x = g;
    let mut y = a;

    let mut res: u64 = 1;
    while y > 0 {
        if y & 1 != 0 {
            res = (res * x) % p;
        }
        y = y >> 1;
        x = (x * x) % p;
    }


    res

}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(p, g, a)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    unimplemented!(
        "Calculate secret key using prime number {}, public key {}, and private key {}",
        p,
        b_pub,
        a
    )
}
