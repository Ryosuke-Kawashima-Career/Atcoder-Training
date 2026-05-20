use proconio::input;
// JOI: 2015
// Q. Span DP
// A. Min max & Span DP
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    // dp[start][length]
    let mut dp = vec![vec![0i64; n + 1]; n];

    for length in 1..n {
        for i in 0..n {
            let j = (i + length - 1) % n;
            let taken = n - length;

            // Second Player's turn
            if taken % 2 == 1 {
                if a[i] > a[j] {
                    dp[i][length] = dp[(i + 1) % n][length - 1];
                } else {
                    dp[i][length] = dp[i][length - 1];
                }
            // First Player's turn
            } else {
                let pick_left = a[i] + dp[(i + 1) % n][length - 1];
                let pick_right = a[j] + dp[i][length - 1];
                dp[i][length] = pick_left.max(pick_right);
            }
        }
    }

    let mut ans = 0i64;
    for i in 0..n {
        ans = ans.max(a[i] + dp[(i + 1) % n][n - 1]);
    }

    println!("{}", ans);
}
