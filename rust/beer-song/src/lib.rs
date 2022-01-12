
pub fn verse(n: u32) -> String {
    let one_bottle = "bottle".to_string();
    let substitute = match n {
        0 => format!("No more {}s", one_bottle),
        1 => format!("1 {}", one_bottle),
        _ => format!("{} {}s", n, one_bottle),
    };
    let first_substring = format!("{0} of beer on the wall, {1} of beer.\n", substitute, substitute.to_lowercase());
    
    let second_substring = match n {
        0 => format!("Go to the store and buy some more"),
        1 => format!("Take it down and pass it around"),
        _ => format!("Take one down and pass it around"),
    };
    let substitute = match n {
        0 => format!("99 {}s", one_bottle),
        1 => format!("no more {}s", one_bottle),
        2 => format!("1 {}", one_bottle),
        _ => format!("{} bottles", n - 1)
    };

    let last_substring = format!(", {} of beer on the wall.\n", substitute);

    format!("{}{}{}", first_substring, second_substring, last_substring)    
}

pub fn sing(start: u32, end: u32) -> String {
    let s: String = (end..=start)
                        .rev()
                        .map(|n| verse(n))
                        .collect::<Vec<String>>()
                        .join("\n");
    s
}
