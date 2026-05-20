use proconio::input;
// Q. Min-Max Game
fn main() {
    input! {n: usize, a: [i64; n]}
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; n + 1]; n + 1];
    for i in 0..n {
        dp[i][i + 1] = a[i];
    }
    let ans = nega_max(0, n, &a, &mut dp);
    println!("{}", ans);
}

fn nega_max(left: usize, right: usize, a: &Vec<i64>, dp: &mut Vec<Vec<i64>>) -> i64 {
    // if visited return the value;
    if dp[left][right] != -1 {
        return dp[left][right];
    }
    let value_left: i64 = a[left] - nega_max(left + 1, right, a, dp);
    let value_right: i64 = a[right - 1] - nega_max(left, right - 1, a, dp);
    dp[left][right] = value_left.max(value_right);
    return dp[left][right];
}
