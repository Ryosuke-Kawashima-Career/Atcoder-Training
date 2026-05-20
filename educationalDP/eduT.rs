use proconio::{input, marker::Chars};
const MOD: usize = 1_000_000_007;
// Educational DP T
// Q. The string S consists of '<' and '>'.
// Find the number of permutations of (1..=n) which satisfy the following conditions:
// For all i in (0..n-1), if S[i] == '<', then p[i] < p[i+1].
// For all i in (0..n-1), if S[i] == '>', then p[i] > p[i+1].
// A. Insertion DP (挿入DP) / Relative Order DP
fn main() {
    input! {n: usize, s: Chars}
    // dp[index][remaining numbers larger than p[index]]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];
    for j in 0..=n {
        dp[0][j] = 1;
    }
    for index in 1..n {
        let mut prefix_sum: Vec<usize> = vec![0; n + 2];
        for larger_count in 1..=n - index + 1 {
            prefix_sum[larger_count] =
                (prefix_sum[larger_count - 1] + dp[index - 1][larger_count - 1]) % MOD;
        }
        for j in 0..=n - index {
            if s[index - 1] == '>' {
                dp[index][j] = prefix_sum[j + 1] % MOD;
            } else {
                dp[index][j] = (prefix_sum[n - index + 1] + MOD - prefix_sum[j + 1]) % MOD;
            }
        }
    }
    // when index=0, there is no number left to compare...
    let ans: usize = dp[n - 1][0];
    println!("{}", ans);
}
