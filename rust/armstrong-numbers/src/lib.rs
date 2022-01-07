pub fn is_armstrong_number(num: u32) -> bool {

    let mut n = num;
    let mut digits = vec![]; 

    loop {
        digits.push(n % 10);
        n /= 10;
        if n == 0 {
            break;
        }
    }
    let l = digits.len() as u32;
    let sum = digits
        .iter()
        .map(|x| x.pow(l))
        .sum();
    
    num == sum
}
