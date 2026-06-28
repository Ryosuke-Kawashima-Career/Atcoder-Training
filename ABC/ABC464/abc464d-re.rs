use proconio::{input, marker::Chars};
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, s: Chars, x: [i64; n], y: [i64; n-1]}
        let (dp_sunny, dp_rainy) = solve_dp(&s, &x, &y);
        let ans: i64 = std::cmp::max(dp_sunny[n - 1], dp_rainy[n - 1]);
        println!("{}", ans);
    }
}

fn solve_dp(s: &[char], x: &[i64], y: &[i64]) -> (Vec<i64>, Vec<i64>) {
    /* Calculates the maximum happiness by DP.
    Args:
        s: Weather sequence (0-indexed, length N)
        x: Daily happiness cost (0-indexed, length N)
        y: Travel reward between days (0-indexed, length N-1)
    Returns:
       dp_sunny[i] = maximum happiness up to day i with s[i] == 'S' (Sunny)
       dp_rainy[i] = maximum happiness up to day i with s[i] == 'R' (Rainy)
    */
    let n = s.len();
    let mut dp_sunny = vec![0i64; n];
    let mut dp_rainy = vec![0i64; n];
    if s[0] == 'S' {
        dp_rainy[0] = -x[0];
    } else {
        dp_sunny[0] = -x[0];
    }
    for i in 1..n {
        if s[i] == 'S' {
            dp_sunny[i] = dp_sunny[i - 1].max(dp_rainy[i - 1] + y[i - 1]);
            dp_rainy[i] = dp_sunny[i - 1].max(dp_rainy[i - 1]) - x[i];
        } else {
            dp_sunny[i] = dp_sunny[i - 1].max(dp_rainy[i - 1] + y[i - 1]) - x[i];
            dp_rainy[i] = dp_sunny[i - 1].max(dp_rainy[i - 1]);
        }
    }
    (dp_sunny, dp_rainy)
}
