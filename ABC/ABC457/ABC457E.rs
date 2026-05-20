use proconio::input;
use std::collections::HashMap;
const INF: usize = 1 << 60;
// ABC457E
// Q. 2 clothes are chosen. Judge wether we can make s[i]..=t[i] covered while the other part is not covered.
// A. Two edges Binary Search
fn main() {
    input! {
        n: usize, m: usize,
        lr: [(usize, usize); m],
        q: usize,
        st: [(usize, usize); q]
    }
    let mut by_l = vec![vec![]; n + 1];
    let mut by_r = vec![vec![]; n + 1];
    let mut min_r_at_l = vec![INF; n + 2];
    let mut ranges = HashMap::new();
    for &(l, r) in &lr {
        *ranges.entry((l, r)).or_insert(0) += 1;
        by_l[l].push(r);
        by_r[r].push(l);
        if r < min_r_at_l[l] {
            min_r_at_l[l] = r;
        }
    }
    // Precompute suffix minimums of min_r_at_l
    let mut suf_min_r = vec![INF; n + 3];
    for l in (0..=n).rev() {
        suf_min_r[l] = min_r_at_l[l].min(suf_min_r[l + 1]);
    }
    for i in 0..=n {
        by_l[i].sort_unstable();
        by_r[i].sort_unstable();
    }
    for query in st {
        if can_cover(&by_l, &by_r, &suf_min_r, &min_r_at_l, &ranges, query) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
fn can_cover(
    by_l: &Vec<Vec<usize>>,
    by_r: &Vec<Vec<usize>>,
    suf_min_r: &[usize],
    min_r_at_l: &[usize],
    ranges: &HashMap<(usize, usize), usize>,
    st: (usize, usize),
) -> bool {
    let (start, end) = st;
    // Case 1: Use one [S, T] piece and any other piece contained in [S, T]
    if let Some(&count) = ranges.get(&st) {
        if count >= 2 {
            return true;
        }
        // Is there any other piece [L, R] such that S <= L and R <= T?
        // Check pieces starting after S
        if suf_min_r[start + 1] <= end {
            return true;
        }
        // Check other pieces starting at S
        if min_r_at_l[start] < end {
            return true;
        }
    }
    // Case 2: Use two pieces [S, R1] and [L2, T] that are NOT [S, T]
    // Find largest R1 < T starting at S
    let pos_r = by_l[start].partition_point(|&x| x < end);
    // Find smallest L2 > S ending at T
    let pos_l = by_r[end].partition_point(|&x| x <= start);
    if pos_r >= 1 && pos_l < by_r[end].len() {
        let r1 = by_l[start][pos_r - 1];
        let l2 = by_r[end][pos_l];
        if l2 <= r1 + 1 {
            return true;
        }
    }
    false
}
