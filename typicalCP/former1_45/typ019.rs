use proconio::input;
const INF: i64 = 1 << 60;
fn main() {
    input! {n: usize, a: [i64; 2*n]}
    let mut dp: Vec<Vec<i64>> = vec![vec![INF; 2 * n + 1]; 2 * n + 1];
    for i in 0..2 * n - 1 {
        dp[i][i + 1] = (a[i + 1] - a[i]).abs();
    }
    for span in (2..=2 * n).step_by(2) {
        for start in 1..=2 * n - span {
            let end: usize = start + span;
            for k in start..end {
                dp[start][end] = dp[start][end].min(dp[start][k] + dp[k][end]);
            }
            dp[start][end] =
                dp[start][end].min(dp[start + 1][end - 1] + (a[start] - a[end - 1]).abs());
        }
    }
    println!("{}", dp[0][2 * n]);
}
