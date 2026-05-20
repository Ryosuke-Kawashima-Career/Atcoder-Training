use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..n {
        input! {u: Usize1, k: usize, v: [Usize1; k]}
        for i in 0..k {
            graph[u].push(v[i]);
        }
    }
    let mut forward: Vec<usize> = vec![n; n];
    let mut backward: Vec<usize> = vec![n; n];
    dfs(0, 0, &graph, &mut forward, &mut backward);
    for id in 0..n {
        println!("{} {} {}", id + 1, forward[id] + 1, backward[id] + 1);
    }
}

fn dfs(
    v: usize,
    current: usize,
    graph: &Vec<Vec<usize>>,
    forward: &mut Vec<usize>,
    backward: &mut Vec<usize>,
) {
    let n: usize = graph.len();
    forward[v] = current;
    let mut max_value: usize = current;
    for &next in graph[v].iter() {
        if forward[next] == n {
            dfs(next, current + 1, graph, forward, backward);
            max_value = max_value.max(backward[next]);
        }
    }
    backward[v] = max_value + 1;
}
