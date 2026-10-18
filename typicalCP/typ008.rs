use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let pattern: Vec<char> = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    let m = pattern.len();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = 1;
    }

    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = dp[i - 1][j];
            if s[i - 1] == pattern[j - 1] {
                dp[i][j] = (dp[i][j] + dp[i - 1][j - 1]) % MOD;
            }
        }
    }

    println!("{}", dp[n][m]);
}
