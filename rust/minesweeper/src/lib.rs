use std::cmp::{min, max, Ordering};

struct MineField<'mf> {
    n: usize,
    m: usize,
    field: Vec<&'mf [i32]>
}

impl<'mf> MineField<'mf> {
    
    pub fn new(minefield: &[&str]) -> Self {
        let n = minefield.len();
        let m = minefield[0].len();

        let v = minefield
            .concat()
            .chars()
            .map(|ch| match ch {
                '*' => -1,
                _ => 0
            })
            .collect::<Vec<_>>();

        let field = v
            .chunks(m)
            .collect::<Vec<_>>();

        let m = MineField {
            n,
            m,
            field: field.clone(),
        };
        // println!("end")
        m
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

    // let is_valid = |i: i32| -> bool {        

    // };

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

    let mut v = minefield
        .concat()
        .chars()
        .map(|ch| match ch {
            '*' => -1,
            _ => 0
        })
        .collect::<Vec<_>>();

    let mut vv = v.chunks(m)
        .collect::<Vec<_>>();

        let mut arr = vec![vec![0i32; n]; m];

    for (ids, s) in minefield.iter().enumerate() {
        for (idc, ch) in s.chars().enumerate() {
            match ch {
                '*' => arr[idc][ids] = -1,
                _ => arr[idc][ids] = 0,
            }
            
        }

    }
    println!("{:?}", arr);
    let _ = minefield
        .iter()
        .enumerate()
        .map(|(ids, &s)|
            for (idc, ch) in s.chars().enumerate() {
                match ch {
                    '*' => arr[idc][ids] += -2,
                    _ => arr[idc][ids] = 0,
                }

            }
        )
        .collect::<Vec<_>>();
    res
}
