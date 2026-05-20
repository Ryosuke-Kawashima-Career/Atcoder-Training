use proconio::input;
const INF: i64 = 1 << 60;
fn main() {
    input! {n: usize, a: [i64; n]}
    let mut dp: Vec<Vec<i64>> = vec![vec![INF; n + 1]; n + 1];
    for left in 0..n {
        dp[left][left + 1] = 0;
    }
    let mut prefix: Vec<i64> = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }
    for length in 2..=n {
        for left in 0..n {
            let right: usize = left + length;
            if right > n {
                break;
            }
            let mut min_cost: i64 = INF;
            for k in left + 1..right {
                let cur_cost: i64 = dp[left][k] + dp[k][right] + prefix[right] - prefix[left];
                min_cost = min_cost.min(cur_cost);
            }
            dp[left][right] = min_cost;
        }
    }
    println!("{}", dp[0][n]);
}
