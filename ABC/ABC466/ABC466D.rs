use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        m: usize,
        rc: [(usize, usize); m],
    }
    let mut rows: Vec<HashSet<usize>> = vec![HashSet::new(); n + 1];
    let mut cols: Vec<HashSet<usize>> = vec![HashSet::new(); n + 1];
    for &(row, col) in rc.iter() {
        rows[row] = HashSet::new();
        cols[col] = HashSet::new();
        rows[row].insert(col);
        cols[col].insert(row);
    }
}
