pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_question = message.ends_with("?");
    let yelling = message.chars()
        .filter(|c| c.is_alphanumeric())
        .filter(|c| !c.is_numeric())
        .collect::<String>();
    let is_yelling = !yelling.is_empty() && yelling.chars().all(|c| c.is_uppercase());

    match message == "" {
        true => "Fine. Be that way!",
        false => match (is_question, is_yelling) {
            (true, false) => "Sure.",
            (true, true) => "Calm down, I know what I'm doing!",
            (false, true) => "Whoa, chill out!",
            (false, false) => "Whatever."
        }
    }
}
