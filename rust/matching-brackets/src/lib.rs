enum Bracket {
    Opened(char),
    Closed(char)
}

impl Bracket {
    pub fn check(c: char) -> Option<Bracket> {
        match c {
            '(' | '[' | '{' => Some(Bracket::Opened(c)),
            ')' => Some(Bracket::Closed('(')),
            ']' => Some(Bracket::Closed('[')),
            '}' => Some(Bracket::Closed('{')),
            _ => None
        }
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for c in string.chars() {
        match Bracket::check(c) {
            Some(Bracket::Opened(br)) => { stack.push(br) },
            Some(Bracket::Closed(br)) if stack.pop() != Some(br) => { return false; }
            _ => {}
        }
    }    
    stack.is_empty()
}
