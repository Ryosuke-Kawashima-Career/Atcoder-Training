use proconio::{input, marker::Chars};
const MOD: usize = 998244353;
// Q. Subsequence(Count the number of subsequences of S that have no adjacent cells of the same letter.)
// A. DP with the last state
fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    // dp[index][char_type] where char_type is 0:a, 1:b, 2:c
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n + 1];
    for i in 1..=n {
        let target_letter: usize = (s[i - 1] as usize - 'a' as usize) % 3;
        let sum_of_other_states: usize =
            (dp[i - 1][(target_letter + 1) % 3] + dp[i - 1][(target_letter + 2) % 3]) % MOD;
        // 1 means only s[i] itself.
        dp[i][target_letter] = (dp[i - 1][target_letter] + sum_of_other_states + 1) % MOD;
        // Inheritance!!!
        dp[i][(target_letter + 1) % 3] = dp[i - 1][(target_letter + 1) % 3];
        dp[i][(target_letter + 2) % 3] = dp[i - 1][(target_letter + 2) % 3];
    }
    let mut ans: usize = 0;
    for j in 0..3 {
        ans = (ans + dp[n][j]) % MOD;
    }
    println!("{}", ans % MOD);
}
