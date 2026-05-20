use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (0, N1), (0, 1), (1, 0)];
// ABC384 E
// Q. The slime at (p, q) absorbs the adjacent slime with 1/x times the size of the slime at (p, q). What is the maximum possible size of the slime at (p, q)?
// A. Greedy Algorithm = Absorb the minimum cell adjacent to the slime.
fn main() {
    input! {h: usize, w: usize, x: usize, p: Usize1, q: Usize1, s: [[usize; w]; h]}
    let max_strength: usize = greedy_simulation(p, q, x, &s);
    println!("{}", max_strength);
}

fn greedy_simulation(p: usize, q: usize, x: usize, s: &Vec<Vec<usize>>) -> usize {
    /*
    Absorbs the minimum cell adjacent to the slime.
    Args:
        p: The initial row of the slime.
        q: The initial column of the slime.
        x: The absorption ratio.
        s: The grid.
    Returns:
        The maximum possible size of the slime.
     */
    let h: usize = s.len();
    let w: usize = s[0].len();
    let mut cur_strength: usize = s[p][q];
    let mut visited = vec![vec![false; w]; h];
    visited[p][q] = true;
    // (cell_value, row, col)
    let mut min_cells = BinaryHeap::new();
    for &(dy, dx) in D4.iter() {
        let next_row: usize = p.wrapping_add(dy);
        let next_col: usize = q.wrapping_add(dx);
        if next_row < h && next_col < w && !visited[next_row][next_col] {
            min_cells.push((Reverse(s[next_row][next_col]), next_row, next_col));
        }
    }
    while let Some((Reverse(cell_value), row, col)) = min_cells.pop() {
        if visited[row][col] {
            continue;
        }
        // (devident + n - 1) / n = ceil(devident / n)
        if cell_value >= (cur_strength + x - 1) / x {
            break;
        }
        cur_strength += cell_value;
        visited[row][col] = true;
        for &(dy, dx) in D4.iter() {
            let next_row: usize = row.wrapping_add(dy);
            let next_col: usize = col.wrapping_add(dx);
            if next_row < h && next_col < w && !visited[next_row][next_col] {
                min_cells.push((Reverse(s[next_row][next_col]), next_row, next_col));
            }
        }
    }
    return cur_strength;
}
