use proconio::input;
const INF: i64 = 1 << 60;
// ABC466E
// Q. Flip the cards of l..r for k times at most and get the maximum sum of the values on the front side of the cards.
// A. Dynamic Programming
fn main() {
    input!{n: usize, k: usize, ab: [(i64, i64); n]}
    let mut potential: Vec<i64> = vec![0; n];
    let mut sum_a: i64 = 0;
    for i in 0..n {
        sum_a = ab[i].0;
        potential[i] = ab[i].1 - ab[i].0;
    }
    // dp[interval][currently in the j-th interval or not]
    let mut dp: Vec<Vec<i64>> = vec![vec![-INF; 2]; k+1];
    for i in 0..n {
        let diff: i64 = potential[i];
        let mut next_dp: Vec<Vec<i64>> = vec![vec![-INF; 2]; k+1];
        for j in 0..=k {
            next_dp[j][0] = dp[j][0].max(dp[j][1]);
            if j == 0 {
                continue;
            }
            let prev_max: i64 = dp[j-1][0].min(dp[j-1][1]);
            let best_prev: i64 = dp[j][1].max(prev_max);
            if best_prev != -INF {
                next_dp[j][1] = best_prev + diff;
            }
        }
        dp = next_dp;
    }
    let mut max_diff: i64 = 0;
    for j in 0..=k {
        max_diff = max_diff.max(dp[j][0]).max(dp[j][1]);
    }
    println!("{}", sum_a + max_diff);
}
