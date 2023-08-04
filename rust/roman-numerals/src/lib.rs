use std::fmt::{Display, Formatter, Result};
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
        let romans = vec!(
            (1000, "M"), (900, "CM"),
            (500, "D"), (400, "CD"),
            (100, "C",), (90, "XC"),
            (50, "L"), (40, "XL"),
            (10, "X"), (9, "IX"),
            (5, "V"), (4, "IV"),
            (1, "I"));
            
        let mut n = num;
        let letters = romans.iter()
            .fold(String::new(), |mut s, (dec, rom)| {
                while n >= *dec {
                    n -= dec;
                    s += *rom;
                }
                s
            });
        Roman {roman: letters}
    }
}
