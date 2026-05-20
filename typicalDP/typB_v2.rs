use proconio::input;
const INF: usize = 1 << 60;
// Q. What is the maximum score that the first player can get?
// A. Minimax DP
/* key takeaways:
Min-Max Game DP must almost always be computed backwards.
DP is calculated from the perspective of the first player.
 */
fn main() {
    input! {na: usize, nb: usize, a: [usize; na], b: [usize; nb]}
    let mut dp: Vec<Vec<usize>> = vec![vec![0; nb + 1]; na + 1];
    for i in (0..=na).rev() {
        for j in (0..=nb).rev() {
            if i == na && j == nb {
                continue;
            }
            if (i + j) % 2 == 0 {
                let mut max_score: usize = 0;
                if i < na {
                    max_score = max_score.max(dp[i + 1][j] + a[i]);
                }
                if j < nb {
                    max_score = max_score.max(dp[i][j + 1] + b[j]);
                }
                dp[i][j] = max_score;
            } else {
                let mut min_score: usize = INF;
                if i < na {
                    min_score = min_score.min(dp[i + 1][j]);
                }
                if j < nb {
                    min_score = min_score.min(dp[i][j + 1]);
                }
                dp[i][j] = min_score;
            }
        }
    }

    println!("{}", dp[0][0]);
}
