use rand::Rng;

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
