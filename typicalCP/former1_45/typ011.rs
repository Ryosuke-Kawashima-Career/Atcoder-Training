use proconio::input;
const MAX_DAY: usize = 5000;
fn main() {
    input! {n: usize, mut dcs: [(usize, usize, i64); n]}
    // sort by due date
    dcs.sort();
    // dp[i: job][j: day] := max value
    let mut dp: Vec<Vec<i64>> = vec![vec![0; MAX_DAY + 1]; n + 1];
    for i in 0..n {
        for day in 0..=MAX_DAY {
            // don't do job_i
            let (due, consecutive, reward) = dcs[i];
            dp[i + 1][day] = dp[i + 1][day].max(dp[i][day]);
            // do job_i
            if day + consecutive <= due {
                dp[i + 1][day + consecutive] =
                    dp[i + 1][day + consecutive].max(dp[i][day] + reward);
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}
