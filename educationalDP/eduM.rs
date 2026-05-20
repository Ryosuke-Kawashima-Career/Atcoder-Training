use proconio::input;

const MOD: i64 = 1_000_000_007;

// 範囲の和を求めるので累積和を用いる
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    // dp[j] := number of ways to distribute exactly j candies among the processed children
    let mut dp = vec![0i64; k + 1];
    dp[0] = 1; // 1 way to distribute 0 candies among 0 children

    for i in 0..n {
        // Build 1-indexed prefix sum for O(1) range sum queries
        // prefix[j] = sum(dp[0..j])
        let mut prefix = vec![0i64; k + 2];
        for j in 0..=k {
            prefix[j + 1] = (prefix[j] + dp[j]) % MOD;
        }

        let mut next_dp = vec![0i64; k + 1];
        let max_candies = a[i];

        for j in 0..=k {
            // To get exactly j candies, the i-th child can receive `c` candies (0 <= c <= a[i]).
            // So we need to sum dp[x] for x from max(0, j - a[i]) to j.
            let right = j;
            let left = if j > max_candies { j - max_candies } else { 0 };

            // Range sum derived from prefix array: prefix[R + 1] - prefix[L]
            next_dp[j] = (prefix[right + 1] + MOD - prefix[left]) % MOD;
        }
        // Move to the next state
        dp = next_dp;
    }

    println!("{}", dp[k]);
}
