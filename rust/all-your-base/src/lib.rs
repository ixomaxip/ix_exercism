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

fn vec_to_dec(digits: &[u32], base: u32) -> Result<u32, Error> {
    if base <= 1 {
        return Err(Error::InvalidInputBase)
    }
    if digits.is_empty() {
        return Ok(0)
    }
    
    let mut mult = u32::pow(base, (digits.len() - 1) as u32);
    let decimal = digits.iter()
        .fold(0, |mut sum, d| {
                dbg!(&d);
                sum += d * mult;
                mult /= base;
                sum
        });

    // match decimal {
    //     Err(e) => return Err(e),

    // }
    Ok(decimal)
}

fn dec_to_vec(dec_number: u32, base: u32) -> Result<Vec<u32>, Error> {
    if base <= 1 {
        return Err(Error::InvalidOutputBase)
    }
    if dec_number == 0 {
        return Ok(vec![0])
    }
    
    let mut digits = vec![];
    let mut decimal = dec_number;
    while decimal > 0 {
        let (q, r) = (decimal / base, decimal % base);
        digits.push(r);
        decimal = q;
    }
    Ok(digits.into_iter().rev().collect())
}

pub fn convert(digits: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let decimal = vec_to_dec(digits, from_base);
    match decimal {
        Err(e) => return Err(e),
        Ok(d) => return dec_to_vec(d, to_base)
    }
}