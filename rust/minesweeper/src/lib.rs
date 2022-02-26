fn split(cols: usize) -> impl Fn(usize) -> (usize, usize) {
    move |i| (i / cols, i % cols)
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let field_size = match minefield.len() {
        0 => return vec![],
        rows => match minefield[0].len() {
            0 => return vec!["".to_string()],
            cols  => (rows, cols)
        }
    };

    let mut field = vec![" ".to_string(); field_size.0 * field_size.1];

    let window_size = (3usize, 3usize);
    let padded_size = (
        field_size.0 + window_size.0 - 1,
        field_size.1 + window_size.1 - 1
    );

    let mut padded_field = vec![0i32; padded_size.0 * padded_size.1];
    for (row, s) in minefield.iter().enumerate() {
        for (col, ch) in s.chars().enumerate() {
            if ch == '*' {
                padded_field[(row + 1) * padded_size.1 + col + 1] = -1;
                field[row * field_size.1 + col] = ch.to_string();
            }
        }
    }

    let _ = (0..field_size.0 * field_size.1)
        .map(split(field_size.1))
        .map(|(row, col)| {
            let borrowed_field = &padded_field;
            (0..window_size.0 * window_size.1)
                .map(split(window_size.1))
                .map(move |(wrow, wcol)| {
                    let idx = (wrow + row) * padded_size.1 + (wcol + col);
                    unsafe { *borrowed_field.get_unchecked(idx) }
                })
            })
        .enumerate().for_each(|(idx, it)| {
            match -it.sum::<i32>() {
                0 => field[idx] = " ".to_string(),
                sum if field[idx] != "*" => field[idx] = sum.to_string(),
                _ => ()
            }
        });

    let field = field
        .join("")
        .chars()
        .collect::<Vec<_>>()
        .chunks(field_size.1)
        .map(|x| x.iter().cloned().collect())
        .collect::<Vec<String>>();

    field
}
