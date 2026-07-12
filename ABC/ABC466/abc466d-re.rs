use proconio::input;
// ABC466D
// Q. Coordinate overupdate
// A. Manage the coordinate cells by rows and cols
// A. Record the last operation of the coordinate grids
fn main() {
    input!{n: usize, m: usize, rc: [(usize, usize); m]}
    let mut last_row: Vec<usize> = vec![0; n+1];
    let mut last_col: Vec<usize> = vec![0; n+1];
    for i in 0..m {
        let (row, col) = rc[i];
        last_row[row] = i;
        last_col[col] = i;
    }
    let mut ans: usize = 0;
    for i in 0..m {
        let (row, col) = rc[i];
        if last_row[row] == i && last_col[col] == i {
            ans += 1;
        }
    }
    println!("{}", ans);
}