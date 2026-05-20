use proconio::input;
// const INF: isize = 1 << 60;
fn main() {
    input! {n: usize, p: [usize; n]}
    let sum_p: usize = p.iter().sum();
    // dp[index][sum of the score]
    let mut dp: Vec<Vec<isize>> = vec![vec![0; sum_p + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for score in 0..=sum_p {
            if score >= p[i - 1] {
                dp[i][score] += dp[i - 1][score - p[i - 1]];
            }
            dp[i][score] += dp[i - 1][score];
        }
    }
    let mut ans: usize = 0;
    for score in 0..=sum_p {
        if dp[n][score] > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
