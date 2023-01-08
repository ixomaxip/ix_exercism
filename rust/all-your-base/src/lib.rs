#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(digits: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // vec_to_dec
    let mut mult = u32::pow(from_base, (digits.len() - 1) as u32);
    let mut dec_number = digits.iter().fold(0, |mut sum, d| {
        sum += d * mult;
        mult /= from_base;
        sum
    });
    dbg!(&dec_number);
    // dec_to_vec
    let mut new_digits = vec![];
    while dec_number > 0 {
        let (q, r) = (dec_number / to_base, dec_number % to_base);
        new_digits.push(r);
        dec_number = q;
    }
    new_digits = new_digits.into_iter().rev().collect();
    dbg!(&new_digits);    
    
    Ok(new_digits)
}
