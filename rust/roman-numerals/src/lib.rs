use std::fmt;
use std::fmt::{Display, Formatter, Result, Debug};
use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;


#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
enum RomanLetters {
    M = 1000,
    CM = 900,
    D = 500,
    CD = 400,
    C = 100,
    XC = 90,
    L = 50,
    XL = 40,
    X = 10,
    IX = 9,
    V = 5,
    IV = 4,
    I = 1
}

impl fmt::Display for RomanLetters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

fn value_to_roman_string(value: usize) -> String {
    match RomanLetters::from_int(value) {
        Ok(color) => color.to_string(),
        Err(_) => String::from("value out of range"),
    }
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        unimplemented!("Return a roman-numeral string representation of the Roman object");
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        unimplemented!("Construct a Roman object from the '{num}' number");
    }
}
