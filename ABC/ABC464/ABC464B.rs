use proconio::{input, marker::Chars};

fn main() {
    input! {h: usize, w: usize, c: [Chars; h]}
    let mut all_white_row: Vec<bool> = vec![true; h];
    let mut all_white_col: Vec<bool> = vec![true; w];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                all_white_row[i] = false;
            }
        }
    }
    for j in 0..w {
        for i in 0..h {
            if c[i][j] == '#' {
                all_white_col[j] = false;
            }
        }
    }
    let mut start_row: usize = 0;
    while all_white_row[start_row] {
        start_row += 1;
    }
    let mut end_row: usize = h - 1;
    while all_white_row[end_row] {
        end_row -= 1;
    }
    let mut start_col: usize = 0;
    while all_white_col[start_col] {
        start_col += 1;
    }
    let mut end_col: usize = w - 1;
    while all_white_col[end_col] {
        end_col -= 1;
    }
    for i in start_row..end_row + 1 {
        for j in start_col..end_col + 1 {
            print!("{}", c[i][j]);
        }
        println!("");
    }
}
