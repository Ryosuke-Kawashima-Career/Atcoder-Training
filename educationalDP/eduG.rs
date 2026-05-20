use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, m: usize, edges: [(Usize1, Usize1); m]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(v1, v2) in edges.iter() {
        graph[v1].push(v2);
    }
    let mut longest_path: Vec<i64> = vec![-1; n];

    for i in 0..n {
        if longest_path[i] == -1 {
            dfs(i, &graph, &mut longest_path);
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(longest_path[i]);
    }
    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<i64>) {
    // if seen
    if dp[v] != -1 {
        return;
    }
    dp[v] = 0;
    for &next in graph[v].iter() {
        dfs(next, graph, dp);
        // Backward update
        dp[v] = dp[v].max(dp[next] + 1);
    }
}
