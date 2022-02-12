
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
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let res: Vec<String> = vec!();
    let n = minefield.len();
    let m = minefield[0].len();

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
