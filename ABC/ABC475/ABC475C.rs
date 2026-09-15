use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;
// get the longest transitions within the length of l.
fn main() {
    input! {n: usize, start: Usize1, l: usize, a: [usize; n-1]}
    // dp[start][end] = (transition, length)
    let mut dp: Vec<Vec<(usize, usize)>> = vec![vec![(0, INF); n]; n];
    dp[start][start].1 = 0;
    dfs(start, 0, l, &a, &mut dp);
    let mut ans: usize = 0;
    for start in 0..n {
        for end in 0..n {
            if dp[start][end].1 <= l {
                ans = ans.max(dp[start][end].0);
            }
        }
    }
    println!("{}", ans);
}

fn dfs(
    start: usize,
    curr_length: usize,
    limit: usize,
    a: &Vec<usize>,
    dp: &mut Vec<Vec<(usize, usize)>>,
) {
    if curr_length > limit {
        return;
    }
    if start > 0 {
        let prev_node: usize = a[start - 1];
        let length_prev: usize = a[prev_node];
        if dp[start][prev_node].0 < dp[prev_node][start].0 + 1 {
            dp[start][prev_node].0 = dp[prev_node][start].0 + 1;
            dp[start][prev_node].1 = dp[prev_node][start].1 + length_prev;
            dfs(prev_node, dp[prev_node][start].1, limit, a, dp);
        } else if dp[start][prev_node].0 == dp[prev_node][start].0 + 1 {
            dp[start][prev_node].1 = dp[start][prev_node]
                .1
                .min(dp[prev_node][start].1 + length_prev);
            dfs(prev_node, dp[prev_node][start].1, limit, a, dp);
        }
    }
    if start + 1 < a.len() {
        let next_node: usize = a[start + 1];
        let length_next: usize = a[next_node];
        if dp[start][next_node].0 < dp[next_node][start].0 + 1 {
            dp[start][next_node].0 = dp[next_node][start].0 + 1;
            dp[start][next_node].1 = dp[next_node][start].1 + length_next;
            dfs(next_node, dp[next_node][start].1, limit, a, dp);
        } else if dp[start][next_node].0 == dp[next_node][start].0 + 1 {
            dp[start][next_node].1 = dp[start][next_node]
                .1
                .min(dp[next_node][start].1 + length_next);
            dfs(next_node, dp[next_node][start].1, limit, a, dp);
        }
    }
}
