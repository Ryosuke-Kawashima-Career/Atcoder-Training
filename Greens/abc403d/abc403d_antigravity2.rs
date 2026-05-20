use proconio::input;
use std::cmp::min;

// abc403d
// Problem: Find minimum deletions such that no two elements have difference D.
// Reference algorithm: Min Weighted Vertex Cover on path graph.
fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let m = 1_000_001; // Max value of A_i is 10^6
    let mut cnt = vec![0; m];
    for &x in &a {
        cnt[x] += 1;
    }

    if d == 0 {
        // If D=0, remove duplicates. Cost = sum(count - 1 for count > 0)
        let mut ans = 0;
        for &c in &cnt {
            if c > 0 {
                ans += c - 1;
            }
        }
        println!("{}", ans);
        return;
    }

    let mut ans = 0;
    // Iterate over remainder classes.
    // We only need to start chains at indices < M.
    let limit = min(d, m);

    for i in 0..limit {
        // Construct the chain: cnt[i], cnt[i+d], cnt[i+2d]... including 0s.
        // This corresponds to Python's cnt[i::D]
        let mut chain = Vec::with_capacity((m - i) / d + 1);
        let mut idx = i;
        while idx < m {
            chain.push(cnt[idx]);
            idx += d;
        }

        ans += calc_dp(&chain);
    }

    println!("{}", ans);
}

// Matches Python's calc(x) logic exactly
fn calc_dp(x: &Vec<usize>) -> usize {
    if x.is_empty() {
        return 0;
    }

    // Python logic maps x -> [0, x[0], x[1]...] (padded with 0 at front)
    // dp[i+1] = min(dp[i] + val[i], dp[i-1] + val[i-1])

    let len = x.len();
    let mut dp = vec![0; len + 2];

    for i in 1..=len {
        let val_current = x[i - 1];
        let val_prev = if i == 1 { 0 } else { x[i - 2] };

        dp[i + 1] = min(dp[i] + val_current, dp[i - 1] + val_prev);
    }

    dp[len + 1]
}
