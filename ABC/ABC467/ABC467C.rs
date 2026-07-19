use proconio::input;
const INF: usize = usize::MAX;
// m = 2
fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; n-1]}
    // a[i] + a[i+1] === b[i] (mod 2)
    // dp[index][added_or_not]
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; 2]; n + 1];
    dp[0][0] = 0;
    for i in 1..n {
        if a[i] + a[i - 1] == b[i - 1] {
            dp[i][0] = dp[i][0].min(dp[i - 1][0]).min(dp[i - 1][1] + 1);
            dp[i][1] = dp[i][1].min(dp[i - 1][0] + 1).min(dp[i - 1][1] + 1);
        } else {
            dp[i][0] = dp[i][0].min(dp[i - 1][0] + 1).min(dp[i - 1][1]);
            dp[i][1] = dp[i][1].min(dp[i - 1][0] + 1).min(dp[i - 1][1]);
        }
    }
    let ans = dp[n - 1][0].min(dp[n - 1][1]);
    println!("{}", ans);
}
