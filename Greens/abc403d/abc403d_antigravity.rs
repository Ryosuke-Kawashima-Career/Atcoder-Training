use proconio::input;
use std::cmp::max;
use std::collections::HashMap;
// abc403d
// Problem: Find minimum deletions such that no two elements have difference D.
// This is equivalent to Total Elements - Maximum Weighted Independent Set on the conflict graph.
// The Maximum Weighted Independent Set (MWIS) is a fundamental problem in graph theory that involves finding a subset of vertices in a weighted undirected graph such that:
// No two vertices are adjacent (meaning no two vertices in the set share an edge).
// The sum of the weights of the selected vertices is as large as possible.
// A. Paraphrasing the problem
// Cxの非ゼロ要素を1つ選んで1減らすことを最小で何回繰り返せば，すべてのxについてCx = 0またはCx + D = 0が成り立つようにできるか？
fn main() {
    input! {
        n: usize,
        d: i64,
        a: [i64; n],
    }

    if d == 0 {
        // If D=0, we cannot have duplicate elements.
        // We can keep at most one of each distinct value.
        let mut map = HashMap::new();
        for &x in &a {
            *map.entry(x).or_insert(0) += 1;
        }
        // Min deletions = Total - Distinct Count
        println!("{}", n - map.len());
        return;
    }

    // Count frequency of each number
    let mut count_map = HashMap::new();
    for &val in &a {
        *count_map.entry(val).or_insert(0) += 1;
    }

    // Identify connected components (chains) based on remainder mod D
    let mut groups: HashMap<i64, Vec<(i64, usize)>> = HashMap::new();
    for (&val, &count) in &count_map {
        let rem = val % d;
        groups.entry(rem).or_default().push((val, count));
    }

    let mut max_kept_total = 0;

    for (_, mut group) in groups {
        // Sort by value to form the chain structure
        group.sort_by_key(|x| x.0);

        let mut i = 0;
        while i < group.len() {
            // Extract a contiguous chain: v, v+D, v+2D...
            // If the gap > D, it breaks the chain.
            let mut chain_counts = vec![group[i].1];
            let mut j = i;
            while j + 1 < group.len() && group[j + 1].0 == group[j].0 + d {
                j += 1;
                chain_counts.push(group[j].1);
            }

            // Solve Max Weighted Independent Set for this chain
            max_kept_total += solve_dp(&chain_counts);

            i = j + 1;
        }
    }

    println!("{}", n - max_kept_total);
}

// DP to find Maximum Weight Independent Set on a path
fn solve_dp(counts: &Vec<usize>) -> usize {
    if counts.is_empty() {
        return 0;
    }

    // dp_incl: max weight ending with current included
    // dp_excl: max weight ending with current excluded
    let mut dp_incl = counts[0];
    let mut dp_excl = 0;

    for &c in counts.iter().skip(1) {
        let new_incl = dp_excl + c;
        let new_excl = max(dp_incl, dp_excl);

        dp_incl = new_incl;
        dp_excl = new_excl;
    }

    max(dp_incl, dp_excl)
}
