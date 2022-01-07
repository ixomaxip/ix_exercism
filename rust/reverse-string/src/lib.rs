// #[cfg(feature = "unicode-segmentation")]
// extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;


pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    // input.chars().rev().collect()
    input.graphemes(true).rev().collect()
}
