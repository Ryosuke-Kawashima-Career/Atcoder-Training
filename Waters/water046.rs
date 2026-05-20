use proconio::input;
const INF: usize = 1 << 60;
fn main() {
    input! {n: usize, matrices: [(usize, usize); n]}
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; n + 1]; n + 1];
    for i in 0..n {
        dp[i][i + 1] = 0;
    }
    for length in 2..=n {
        for start in 0..=n - length {
            let end: usize = start + length;
            // start + 1 is necessary!!!
            for k in start + 1..end {
                dp[start][end] = dp[start][end].min(
                    dp[start][k]
                        + dp[k][end]
                        + matrices[start].0 * matrices[k].0 * matrices[end - 1].1,
                );
            }
        }
    }
    println!("{}", dp[0][n]);
}
