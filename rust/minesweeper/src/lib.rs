pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let res: Vec<String> = vec!();

    let mut v = minefield
        .concat()
        .chars()
        .map(|ch| match ch {
            '*' => -1,
            _ => 0
        })
        .collect::<Vec<_>>();

    let dim = Dim {
        n: minefield.len(),
        m: v.len() / minefield.len()
    };
    res
}
