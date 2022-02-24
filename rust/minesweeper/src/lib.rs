struct MineField {
    rows: usize,
    cols: usize,
    field: Vec<i32>,
}

fn split(cols: usize) -> impl Fn(usize) -> (usize, usize) {
    move |i| (i / cols, i % cols)
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    let cols = minefield[0].len();

    let mut field = vec![" ".to_string(); cols * rows];

    let window_size = (3usize, 3usize);    
    let padded_rows = rows + window_size.0 - 1;
    let padded_cols = cols + window_size.1 - 1;

    let mut padded_field = vec![0i32; padded_rows * padded_cols];
    for (row, s) in minefield.iter().enumerate() {
        for (col, ch) in s.chars().enumerate() {
            if ch == '*' {
                padded_field[(row + 1) * padded_cols + col + 1] = -1;
                field[row * cols + col] = ch.to_string();
            }
        }
    }

    let win_iter = (0..rows * cols)
        .map(split(cols))
        .map(|(row, col)| {
            let borrowed_field = &padded_field;
            (0..window_size.0 * window_size.1)
                .map(split(window_size.1))
                .map(move |(wrow, wcol)| {
                    let idx = (wrow + row) * padded_cols + (wcol + col);
                    unsafe { *borrowed_field.get_unchecked(idx)}
                })
            })
        .enumerate().for_each(|(idx, it)| {
            match -it.sum::<i32>() {
                0 => field[idx] = " ".to_string(),
                sum if field[idx] != "*" => field[idx] = sum.to_string(),
                _ => ()
            }
        });

    // for debug 
    let v = field
        .chunks(cols)
        .collect::<Vec<_>>();
    field
}
