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

pub struct Roman {
    roman: String
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.roman)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut n = num;
        let letters = RomanLetters::into_enum_iter()
            .fold(String::new(), |mut s, l| {
                let val = l.int_value() as u32;                
                while n >= val {
                    n -= val;
                    dbg!(n);
                    s += &l.to_string();
                }
                s
            });
        Roman {roman: letters}
    }
}
