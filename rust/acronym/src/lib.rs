pub fn abbreviate(phrase: &str) -> String {

    let res = phrase
        .split(" ")
        .map(|s| {            
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase()
                .chain(chars).collect()
            }

        })
        .collect::<String>()
        .chars()
        .filter(|c| c.is_uppercase())
        .collect::<String>();


    dbg!(res);
    String::new()
}
