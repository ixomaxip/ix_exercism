pub fn reply(message: &str) -> &str {
    let replies = vec!(
        "Sure.",
        "Whoa, chill out!",
        "Calm down, I know what I'm doing!",
        "Fine. Be that way!",
        "Whatever."

    );

    let message = message.trim();
    let yelling = message.chars()
        .filter(|c| c.is_alphanumeric())
        .filter(|c| !c.is_numeric())
        .collect::<String>();
        
        
    let is_question = message.ends_with("?");
    // let uppercase = (b'A'..b'Z').map(|c| c as char).collect::<String>();
    // let is_yelling = yelling.chars().all(|c| uppercase.contains(c));
    let is_yelling = yelling.chars().all(|c| c.is_uppercase());
    // dbg!(message);
    // dbg!(yelling);
    // dbg!(is_yelling);
    match message == "" {
        true => replies[3],
        false => match is_yelling {
            true => match is_question {
                true => replies[2],
                false => replies[1],
            },
            false => match is_question {
                true => replies[0],
                false => replies[4],
            }
        }
    }

    // replies[4]
}
