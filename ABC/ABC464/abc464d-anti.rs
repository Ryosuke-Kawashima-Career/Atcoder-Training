use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
            x: [i64; n],
            y: [i64; n - 1],
        }

        // dp[0]: last day was Sunny ('S')
        // dp[1]: last day was Rainy ('R')
        let mut dp = [0i64; 2];

        // Day 0 initialization
        if s[0] == 'S' {
            dp[0] = 0;
            dp[1] = -x[0];
        } else {
            dp[0] = -x[0];
            dp[1] = 0;
        }

        // Transitions for day 1 to n - 1
        for i in 1..n {
            let (c_s, c_r) = if s[i] == 'S' { (0, -x[i]) } else { (-x[i], 0) };

            let next_dp0 = std::cmp::max(dp[0], dp[1] + y[i - 1]) + c_s;
            let next_dp1 = std::cmp::max(dp[0], dp[1]) + c_r;

            dp[0] = next_dp0;
            dp[1] = next_dp1;
        }

        let ans = std::cmp::max(dp[0], dp[1]);
        println!("{}", ans);
    }
}
