use proconio::input;
use std::collections::BinaryHeap;
use std::collections::HashSet;
// ABC391F
// Q. For each of the N^3choices of integers i,j,k, compute the valueAi*Bj+Bj*Ck+Ck*Ai. Among all these values, find the K-th largest
// A. Compromise Simulation by binary tree
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }
    a.sort_by(|x1, x2| x2.cmp(&x1));
    b.sort_by(|x1, x2| x2.cmp(&x1));
    c.sort_by(|x1, x2| x2.cmp(&x1));
    // (score, (index_a, index_b, index_c))
    let mut best = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut top_k: Vec<usize> = Vec::new();
    let best_score: usize = a[0] * b[0] + b[0] * c[0] + c[0] * a[0];
    let initial_indexes = (0, 0, 0);
    best.push((best_score, initial_indexes));
    visited.insert(initial_indexes);
    while let Some((score, (index_a, index_b, index_c))) = best.pop() {
        top_k.push(score);
        if top_k.len() == k {
            break;
        }
        let next_indexes = [
            (index_a + 1, index_b, index_c),
            (index_a, index_b + 1, index_c),
            (index_a, index_b, index_c + 1),
        ];
        for (index_a, index_b, index_c) in next_indexes {
            if index_a < n && index_b < n && index_c < n {
                let next_score =
                    a[index_a] * b[index_b] + b[index_b] * c[index_c] + c[index_c] * a[index_a];
                if !visited.contains(&(index_a, index_b, index_c)) {
                    best.push((next_score, (index_a, index_b, index_c)));
                    visited.insert((index_a, index_b, index_c));
                }
            }
        }
    }
    println!("{}", top_k[k - 1]);
}
