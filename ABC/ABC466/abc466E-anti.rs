use proconio::input;

const INF: i64 = 1e16 as i64;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(i64, i64); n],
    }

    let mut c = vec![0; n];
    let mut sum_a = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        c[i] = b - a;
        sum_a += a;
    }

    // dp[j][0]: at most j intervals, not currently in an interval
    // dp[j][1]: at most j intervals, currently in the j-th interval
    let mut dp = vec![[-INF; 2]; k + 1];
    dp[0][0] = 0;

    for i in 0..n {
        let val = c[i];
        let mut next_dp = vec![[-INF; 2]; k + 1];

        for j in 0..=k {
            next_dp[j][0] = std::cmp::max(dp[j][0], dp[j][1]);

            if j >= 1 {
                let prev_max = std::cmp::max(dp[j - 1][0], dp[j - 1][1]);
                let best_prev = std::cmp::max(dp[j][1], prev_max);
                if best_prev > -INF {
                    next_dp[j][1] = best_prev + val;
                }
            }
        }
        dp = next_dp;
    }

    let mut max_diff = 0;
    for j in 0..=k {
        max_diff = std::cmp::max(max_diff, dp[j][0]);
        max_diff = std::cmp::max(max_diff, dp[j][1]);
    }

    println!("{}", sum_a + max_diff);
}
