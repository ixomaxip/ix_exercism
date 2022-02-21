fn get_primes(limit: u32) -> Vec<usize> {
    // let limit = limit as usize;
    let mut primes: Vec<bool> = (0..=limit)
        .map(|num| num == 2 || num & 1 != 0)
        .collect();

    let mut num = 3usize;
    let limit = limit as usize;
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

pub fn nth(n: u32) -> u32 {
    let mut total: u32 = 0;
    let mut bin_size = 10;
    let mut primes: Vec<usize> = vec!();
    while total <= n {
        primes = get_primes(bin_size);
        total = primes.len() as u32;
        bin_size += n;
    }    
    primes[n as usize]  as u32
}
