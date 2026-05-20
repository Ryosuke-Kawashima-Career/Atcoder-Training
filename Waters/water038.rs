use proconio::{input, marker::Chars};

fn main() {
    input! {q: usize}
    for _query in 0..q {
        input! {s: Chars, t: Chars}
        let n = s.len();
        let m = t.len();
        let mut dp: Vec<Vec<usize>> = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=m {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - 1]);
                }
                dp[i][j] = dp[i][j].max(dp[i - 1][j]);
                dp[i][j] = dp[i][j].max(dp[i][j - 1]);
            }
        }
        println!("{}", dp[n][m])
    }
}
