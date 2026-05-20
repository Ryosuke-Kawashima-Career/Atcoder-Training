use proconio::input;
// keypoint: the number of a_i <= 3
fn main() {
    input! {n: usize, a: [usize; n]}
    // dp[remaining_1][remaining_2][remaining_3]
    let mut dp: Vec<Vec<Vec<f64>>> = vec![vec![vec![0.0; n + 1]; n + 1]; n + 1];
    let mut count: [usize; 4] = [0; 4];
    for i in 0..n {
        count[a[i]] += 1;
    }
    for k in 0..=count[3] {
        for j in 0..=n {
            for i in 0..=n {
                if i + j + k > n || i + j + k == 0 {
                    continue;
                }
                dp[i][j][k] = 1.0;
                // time flow: left -> right
                if i >= 1 {
                    dp[i][j][k] += dp[i - 1][j][k] * (i as f64) / (n as f64);
                }
                if j >= 1 {
                    dp[i][j][k] += dp[i + 1][j - 1][k] * (j as f64) / (n as f64);
                }
                if k >= 1 {
                    dp[i][j][k] += dp[i][j + 1][k - 1] * (k as f64) / (n as f64);
                }
                dp[i][j][k] *= n as f64 / (i + j + k) as f64;
            }
        }
    }
    println!("{:.10}", dp[count[1]][count[2]][count[3]]);
}
