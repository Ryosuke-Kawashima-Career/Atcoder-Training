use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, q: usize, edges: [(Usize1, Usize1); q]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }
}
