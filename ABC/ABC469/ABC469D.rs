use proconio::{input, marker::Usize1};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut count = vec![0usize; n];
    let mut pair_count: HashMap<(usize, usize), usize> = HashMap::new();

    for &(a, b) in ab.iter() {
        count[a] += 1;
        count[b] += 1;
        let u = a.min(b);
        let v = a.max(b);
        *pair_count.entry((u, v)).or_insert(0) += 1;
    }

    // Find missed tournaments for player A_0 and player B_0
    let a0 = ab[0].0;
    let b0 = ab[0].1;

    let mut missed_a0 = Vec::new();
    let mut missed_b0 = Vec::new();

    for (idx, &(a, b)) in ab.iter().enumerate() {
        if a != a0 && b != a0 {
            missed_a0.push(idx);
        }
        if a != b0 && b != b0 {
            missed_b0.push(idx);
        }
    }

    let is_valid = |u: usize, v: usize| -> bool {
        let (p1, p2) = (u.min(v), u.max(v));
        let common = pair_count.get(&(p1, p2)).copied().unwrap_or(0);
        count[u] + count[v] - common == m
    };

    let mut valid_pairs = HashSet::new();

    // Process anchor u = a0
    if missed_a0.is_empty() {
        for v in 0..n {
            if v != a0 {
                valid_pairs.insert((a0.min(v), a0.max(v)));
            }
        }
    } else {
        let first_missed_idx = missed_a0[0];
        let (c1, c2) = ab[first_missed_idx];
        if c1 != a0 && is_valid(a0, c1) {
            valid_pairs.insert((a0.min(c1), a0.max(c1)));
        }
        if c2 != a0 && is_valid(a0, c2) {
            valid_pairs.insert((a0.min(c2), a0.max(c2)));
        }
    }

    // Process anchor u = b0
    if missed_b0.is_empty() {
        for v in 0..n {
            if v != b0 {
                valid_pairs.insert((b0.min(v), b0.max(v)));
            }
        }
    } else {
        let first_missed_idx = missed_b0[0];
        let (c1, c2) = ab[first_missed_idx];
        if c1 != b0 && is_valid(b0, c1) {
            valid_pairs.insert((b0.min(c1), b0.max(c1)));
        }
        if c2 != b0 && is_valid(b0, c2) {
            valid_pairs.insert((b0.min(c2), b0.max(c2)));
        }
    }

    println!("{}", valid_pairs.len());
}
