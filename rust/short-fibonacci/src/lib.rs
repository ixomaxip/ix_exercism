/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let n: usize = 5;
    let mut zeros = create_buffer(n);
    for i in 0..zeros.len() {
        match i {
            0 | 1 => zeros[i] = 1,
            _ => zeros[i] = &zeros[i - 1] + &zeros[i - 2]
        }
    }
    zeros
    // let mut prev: u8 = 1;
    // let mut curr: u8 = 1;
 
    // let _ = zeros
    //     .iter_mut()
    //     .enumerate()
    //     .map(|(i, val)| {
    //         match i {
    //             0 | 1 => *val = 1,
    //             _ => {
    //                 *val = curr + prev;
    //                 prev = curr;
    //                 curr = *val;
    //             }
    //         }
    //     })
    //     .collect::<Vec<_>>();    
    // zeros.to_vec()

}
