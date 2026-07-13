use proconio::input;
// ABC466D
// Q. Coordinate overupdate
// A. Manage the coordinate cells by rows and cols
// A. Focus on the constraint of the problem: There is only one pawn on each row and column at most.
fn main() {
    input! {n: usize, m: usize, rc: [(usize, usize); m]}
    let mut row_state: Vec<usize> = vec![0; n + 1];
    let mut col_state: Vec<usize> = vec![0; n + 1];
    for &(r, c) in rc.iter() {
        let cur_col_of_row: usize = row_state[r];
        let cur_row_of_col: usize = col_state[c];
        if cur_col_of_row != 0 {
            // Erase the existing pawn
            col_state[cur_col_of_row] = 0;
        }
        if cur_row_of_col != 0 {
            // Erase the existing pawn
            row_state[cur_row_of_col] = 0;
        }
        row_state[r] = c;
        col_state[c] = r;
    }
    let mut ans: usize = 0;
    for row in 1..=n {
        if row_state[row] != 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
