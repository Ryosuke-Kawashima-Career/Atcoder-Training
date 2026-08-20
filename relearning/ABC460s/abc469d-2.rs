use proconio::{input, marker::Usize1};
use std::collections::HashMap;
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1); m]}
    let mut cnt1: HashMap<usize, usize> = HashMap::new();
    let mut cnt2: HashMap<(usize, usize), usize> = HashMap::new();
    for &(a, b) in edges.iter() {
        *cnt1.entry(a).or_insert(0) += 1;
        *cnt1.entry(b).or_insert(0) += 1;
        let p = if a < b { (a, b) } else { (b, a) };
        *cnt2.entry(p).or_insert(0) += 1;
    }
    let mut count: usize = 0;
    let p: (usize, usize) = if edges[0].0 < edges[0].1 {
        (edges[0].0, edges[0].1)
    } else {
        (edges[0].1, edges[0].0)
    };
    let mut count: usize = if cnt1[&p.0] + cnt1[&p.1] - cnt2[&p] == m {
        1
    } else {
        0
    };
    for player in 0..n {
        if player == p.0 || player == p.1 {
            continue;
        }
        if let Some(comb) = cnt2.get(&(player.min(p.0), player.max(p.0))) {
            if cnt1[&player] + cnt1[&p.0] - *comb == m {
                count += 1;
            }
        }
        if let Some(comb) = cnt2.get(&(player.min(p.1), player.max(p.1))) {
            if cnt1[&player] + cnt1[&p.1] - *comb == m {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
