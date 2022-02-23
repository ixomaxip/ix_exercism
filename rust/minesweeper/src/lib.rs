use std::cmp::{min, max, Ordering};


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
        let rows = minefield.len();
        let cols = minefield[0].len();

        // let mut field = vec![vec![0i32; cols]; rows];
        // for (ids, s) in minefield.iter().enumerate() {
        //     for (idc, ch) in s.chars().enumerate() {
        //         match ch {
        //             '*' => field[ids][idc] = -1,
        //             _ => field[ids][idc] = 0,
        //         }            
        //     }
        // }    

        let field = minefield
            .concat()
            .chars()
            .map(|ch| match ch {
                '*' => -1,
                _ => 0
            })
            .collect::<Vec<_>>();

        // let field = v
        //     .chunks(m)
        //     .collect::<Vec<_>>();


        let window_size = (3usize, 3usize);

        let trimmed_field = (
            rows - window_size.0 + 1,
            cols - window_size.1 + 1
        );


        let win_iter = (0..trimmed_field.0 * trimmed_field.1)
            .map(Self::split(trimmed_field.1))
            .map(|(row, col)| {
                let borrowed_field = &field;
                return (0..window_size.0 * window_size.1)
                    .map(Self::split(window_size.1))
                    .map(move |(wrow, wcol)| {
                        let idx = (wrow + row) * cols + (wcol + col);
                        (idx, unsafe { *borrowed_field.get_unchecked(idx)})
                    });
                });

        win_iter.for_each(|it| println!("{:?}", it.collect::<Vec<(usize, i32)>>()));

        MineField {
            rows,
            cols,
            field
        }
    }
}

fn get_idxs(idx: usize, n: usize, m: usize) -> Vec<usize> {
    
    let res: Vec<usize> = vec![];
    //    <------m------>     
    // -    -    -    -    -  |
    // -    ul   u    ur   -  n
    // -    l   idx   r    -  |
    // -    dl   d   dr    -  |


    // -    -    -    -    -

    let m = m as i32;
    let n = n as i32;

    let i = idx as i32;
    let l: i32 = match ((i - 1) / m).cmp(&(i / m)) {
        Ordering::Equal => i - 1,
        _ => -1
    };
    let r: i32 = match ((i + 1) / m).cmp(&(i / m)) {
        Ordering::Equal => i + 1,
        _ => -1
    };

    let u: i32 = i - m;
    let ul: i32 = u - 1;
    let ur: i32 = u + 1;

    let d: i32 = i + m;
    let dl: i32 = d - 1;
    let dr: i32 = d + 1;

    res
}

fn find_mines(i: usize, j: usize, v: &mut Vec<&mut [i32]>) {
    let mask = vec!((-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1));

    for dir in mask {
        let i_check = i as i32 + dir.0;
        let j_check = j as i32 + dir.1;
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
