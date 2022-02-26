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

fn main() {

    let test_case: &[&str] = &[
        "1*3*1",
        "13*31",
        " 2*2 ",
        " 111 "
    ];
    // let test_case: &[&str] = &[];
    
    let cleaned = remove_annotations(test_case);
    println!("cleaned = {:?}", cleaned);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    println!("cleaned_strs = {:?}", cleaned_strs);
    
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    println!("expected = {:?}", expected);
    let res = annotate(&cleaned_strs);

    println!("result = {:?}", res);
}
