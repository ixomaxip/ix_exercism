// use anyhow::{Context, Result};

use minesweeper::annotate;

fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() {

    let test_case: &[&str] = &[
        "1*3*1",
        "13*31",
        " 2*2 ",
        " 111 "
    ];

    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();

    let res = annotate(&cleaned_strs);

    println!("{:?}", res);
}
