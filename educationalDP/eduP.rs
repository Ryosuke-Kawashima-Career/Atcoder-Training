use proconio::{input, marker::Usize1};
const MOD: usize = 1_000_000_007;
const NIL: usize = usize::MAX;
fn main() {
    input! {n: usize, edges: [(Usize1, Usize1); n - 1]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; n];
    for &(v1, v2) in edges.iter() {
        graph[v1].push(v2);
        graph[v2].push(v1);
    }
    dfs(0, NIL, &graph, &mut dp);
    println!("{}", (dp[0][0] + dp[0][1]) % MOD);
}

fn dfs(v: usize, p: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<Vec<usize>>) {
    // dp[v][0]: v is white
    // dp[v][1]: v is black
    dp[v][0] = 1;
    dp[v][1] = 1;
    for &u in graph[v].iter() {
        if u == p {
            continue;
        }
        dfs(u, v, graph, dp);
        dp[v][0] = (dp[v][0] * (dp[u][0] + dp[u][1])) % MOD;
        dp[v][1] = (dp[v][1] * dp[u][0]) % MOD;
    }
}
