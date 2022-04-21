pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        _ => format!("{1}And all for the want of a {0}.",
            list[0],
            list.windows(2).fold(String::new(), |acc, w| {
                format!("{}For want of a {} the {} was lost.\n", acc, w[0], w[1])
            })
        )
    }
}