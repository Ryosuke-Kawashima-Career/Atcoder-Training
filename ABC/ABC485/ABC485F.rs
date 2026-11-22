use proconio::input;

const NEG_INF: i64 = -(1 << 60);
// A. Sorting + Shifted-Reference Knapsack DP
fn main() {
    input! {
        n: usize,
        m: usize,
        mut cv: [(usize, i64); n],
    }

    // Sort items in descending order of cost c_i
    cv.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    // suffix_val[k] = sum of values of items k..n (which have cost 0 when j >= c_k)
    let mut suffix_val: Vec<i64> = vec![0; n + 1];
    for k in (0..n).rev() {
        suffix_val[k] = suffix_val[k + 1] + cv[k].1;
    }

    // dp[t][w] = max value of choosing t items from 0..=k with sum(c_i - c_k) <= w
    let mut dp: Vec<Vec<i64>> = vec![vec![NEG_INF; m + 1]; n + 1];
    dp[0].fill(0);

    let mut ans: Vec<i64> = vec![0; m + 1];
    for j in cv[0].0..=m {
        ans[j] = suffix_val[0];
    }

    for k in 0..n {
        let (c_k, v_k) = cv[k];

        if k > 0 {
            let delta = cv[k - 1].0 - c_k;
            if delta > 0 {
                for t in 1..=k {
                    let shift = t * delta;
                    if shift > m {
                        dp[t].fill(NEG_INF);
                    } else {
                        for w in (shift..=m).rev() {
                            dp[t][w] = dp[t][w - shift];
                        }
                        for w in 0..shift {
                            dp[t][w] = NEG_INF;
                        }
                    }
                }
            }
        }

        // Insert item k (whose shifted cost c_k - c_k is 0)
        for t in (0..=k).rev() {
            for w in 0..=m {
                let prev = dp[t][w];
                if prev != NEG_INF {
                    let cand = prev + v_k;
                    if cand > dp[t + 1][w] {
                        dp[t + 1][w] = cand;
                    }
                }
            }
        }

        let next_c = if k + 1 < n { cv[k + 1].0 } else { 0 };
        for j in next_c..c_k {
            let d = c_k - j;
            let mut best = 0;
            for t in 0..=k + 1 {
                if t * d > m {
                    break;
                }
                best = best.max(dp[t][m - t * d]);
            }
            ans[j] = suffix_val[k + 1] + best;
        }
    }

    for j in 0..=m {
        println!("{}", ans[j]);
    }
}
