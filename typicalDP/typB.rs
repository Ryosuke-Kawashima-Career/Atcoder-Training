use proconio::input;
// Q. What is the definition of the optimal policy?
//   - Is this about maximizing the score at hand.
// A.
fn main() {
    input! {na: usize, nb: usize, a: [usize; na], b: [usize; nb]}
    let mut dp: Vec<Vec<usize>> = vec![vec![0; nb + 1]; na + 1];
    for i in 1..=na {
        for j in 1..=nb {
            if i >= 2 {
                dp[i][j] = dp[i][j].max(dp[i - 2][j] + a[i - 2]);
            }
            if j >= 2 {
                dp[i][j] = dp[i][j].max(dp[i][j - 2] + b[j - 2]);
            }
            if i >= 1 && j >= 1 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + a[i - 1] + b[j - 1]);
            }
        }
    }
    let ans = if (na + nb) % 2 == 0 {
        dp[na][nb]
    } else {
        let mut candidate = dp[na - 1][nb - 1];
        if na >= 2 {
            candidate = candidate.max(dp[na - 2][nb]);
        }
        if nb >= 2 {
            candidate = candidate.max(dp[na][nb - 2]);
        }
        candidate
    };
    println!("{}", ans);
}
