pub fn abbreviate(phrase: &str) -> String {

    let res = phrase.replace(|c: char| !c.is_alphanumeric() && c != '\'', " ")
        .split(" ").filter(|&x| !x.is_empty())
        .map(|s| {
            let mut chars = s.chars(); 
            if chars.all(|c| c.is_uppercase()) {
                s.to_lowercase().to_string()
            } else {
                s.to_string()
            }
        })
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().chain(chars).collect()
            }

        })
        .collect::<String>()
        .chars()
        .filter(|c| c.is_uppercase())
        .collect::<String>();
    res
}
