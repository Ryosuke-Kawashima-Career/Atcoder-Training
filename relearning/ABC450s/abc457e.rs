use proconio::input;
const INF: usize = 1 << 60;
use std::collections::HashMap;
fn main() {
    input! {n: usize, m: usize, lr: [(usize, usize); m], q: usize, st: [(usize, usize); q]}
    let mut by_l: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut by_r: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut min_r_at_l: Vec<usize> = vec![INF; n + 2];
    let mut count_range: HashMap<(usize, usize), usize> = HashMap::new();
    for &(l, r) in lr.iter() {
        by_l[l].push(r);
        by_r[r].push(l);
        min_r_at_l[l] = min_r_at_l[l].min(r);
        *count_range.entry((l, r)).or_insert(0) += 1;
    }
    // do not forget about sorting
    for i in 0..=n {
        by_l[i].sort();
        by_r[i].sort();
    }
    let mut suf_min_r: Vec<usize> = vec![INF; n + 3];
    for i in (0..(n + 2)).rev() {
        suf_min_r[i] = suf_min_r[i + 1].min(min_r_at_l[i]);
    }

    let mut answer: Vec<String> = Vec::new();
    for query in 0..q {
        let (left_edge, right_edge) = st[query];
        // Pattern 1: There is a range [L, R] exactly matching [left_edge, right_edge]
        if let Some(&count) = count_range.get(&(left_edge, right_edge)) {
            if count >= 2 {
                answer.push("Yes".to_string());
                continue;
            } else {
                let min_r_from_left = suf_min_r[left_edge];
                if min_r_from_left <= right_edge {
                    answer.push("Yes".to_string());
                    continue;
                } else {
                    answer.push("No".to_string());
                    continue;
                }
            }
        }
        // Pattern2: Both clothes cover left or right
        let pos_r: usize = by_l[left_edge].partition_point(|&r| r < right_edge);
        let pos_l: usize = by_r[right_edge].partition_point(|&l| l <= left_edge);

        if by_r[right_edge][pos_l] <= by_l[left_edge][pos_r - 1] + 1 {
            answer.push("Yes".to_string());
            continue;
        } else {
            false;
        }
    }
    println!("{}", answer.join("\n"));
}
