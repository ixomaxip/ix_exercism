// use std::cmp::{Ordering};


struct MineField {
    rows: usize,
    cols: usize,
    field: Vec<i32>,
}

impl MineField {

    fn linearize(self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn split(cols: usize) -> impl Fn(usize) -> (usize, usize) {
        move |i| (i / cols, i % cols)
    }
    
    pub fn new(minefield: &[&str]) -> MineField {

        let rows = minefield.len() + 2;
        let cols = minefield[0].len() + 2;

        let mut field = vec![0i32; (cols - 2) * (rows - 2)];
        let mut padded_field = vec![0i32; cols* rows];
        for (row, s) in minefield.iter().enumerate() {
            for (col, ch) in s.chars().enumerate() {
                if ch == '*' {
                    padded_field[(row + 1) * cols + col + 1] = -1;
                    field[row * (cols - 2) + col] = -1;
                }
            }
        }

        // for debug 
        let v = field
            .chunks(cols - 2)
            .collect::<Vec<_>>();


        let window_size = (3usize, 3usize);
        let trimmed_field = (
            rows - window_size.0 + 1,
            cols - window_size.1 + 1
        );

        let win_iter = (0..trimmed_field.0 * trimmed_field.1)
            .map(Self::split(trimmed_field.1))
            .map(|(row, col)| {
                let borrowed_field = &padded_field;
                (0..window_size.0 * window_size.1)
                    .map(Self::split(window_size.1))
                    .map(move |(wrow, wcol)| {
                        let idx = (wrow + row) * cols + (wcol + col);
                        unsafe { *borrowed_field.get_unchecked(idx)}
                    })
                });

        // win_iter.for_each(|it| println!("{:?}", it.collect::<Vec<(usize, i32)>>()));
        // win_iter.for_each(|it| println!("{:?}", it.collect::<Vec<i32>>()));
        win_iter.for_each(|it| println!("{:?}", it.sum::<i32>()));
        // win_iter.for_each(|it| println!("{:?}", it.map(| (i, val)| sum()));

        MineField {
            rows,
            cols,
            field
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let res: Vec<String> = vec!();

    let n = minefield.len();
    let m = minefield[0].len();

    let field = MineField::new(minefield);


    println!("{:?}", field.field);
    res
}
