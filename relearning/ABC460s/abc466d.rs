use proconio::input;
// Note that there is only one pawn at most on each row.
fn main() {
    input! {n: usize, m: usize, rc: [(usize, usize); m]}
    let mut rows: Vec<usize> = vec![0; n + 1];
    let mut cols: Vec<usize> = vec![0; n + 1];
    for query in 0..m {
        let (row, col) = rc[query];
        let mut cur_col: usize = rows[row];
        cols[cur_col] = 0;
        rows[row] = col;
        let mut cur_row: usize = cols[col];
        cols[col] = row;
        rows[cur_row] = 0;
    }

    let mut ans: usize = 0;
    for row in 1..=n {
        let cur_col: usize = rows[row];
        if cur_col != 0 && cols[cur_col] != 0 && row == cols[cur_col] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
