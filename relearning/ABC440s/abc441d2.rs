use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, m: usize, l: usize, s: i64, t: i64, uvc: [(Usize1, Usize1, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(u, v, w) in uvc.iter() {
        graph[u].push((v, w));
    }
    let mut is_ok: Vec<bool> = vec![false; n];
    dfs(0, 0, 0, l, s, t, &graph, &mut is_ok);
    for v in 0..n {
        if is_ok[v] {
            print!("{} ", v + 1);
        }
    }
    print!("");
}

fn dfs(
    curr_v: usize,
    phase: usize,
    cost: i64,
    l: usize,
    s: i64,
    t: i64,
    graph: &Vec<Vec<(usize, i64)>>,
    is_ok: &mut Vec<bool>,
) {
    // pruning at the initial stage
    if cost > t {
        return;
    }
    if is_ok[curr_v] {
        return;
    }
    if phase == l {
        if s <= cost && cost <= t {
            is_ok[curr_v] = true;
        }
        return;
    }
    // move to next nodes
    for &(next_v, w) in graph[curr_v].iter() {
        let next_cost: i64 = cost + w;
        dfs(next_v, phase + 1, next_cost, l, s, t, graph, is_ok);
    }
}
