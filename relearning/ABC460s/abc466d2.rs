use proconio::input;
// Reverse the queries
fn main() {
    input! {n: usize, m: usize, rc: [(usize, usize); m]}
    let mut is_row_used: Vec<bool> = vec![false; n + 1];
    let mut is_col_used: Vec<bool> = vec![false; n + 1];
    let mut ans: usize = 0;
    for query in (0..m).rev() {
        let (row, col) = rc[query];
        if !is_row_used[row] && !is_col_used[col] {
            ans += 1;
        }
        is_row_used[row] = true;
        is_col_used[col] = true;
    }
    println!("{}", ans);
}
