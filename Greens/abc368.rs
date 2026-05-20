use proconio::{input, marker::Usize1};
use std::collections::HashSet;
// ABC368 D: Minimum Steiner Tree
// Q. Find the minimum sized tree that contains all the vertices in v.
// A. Depth First Search: Get the existence of the target vertices by the bottom-up order.
fn main() {
    input! {n: usize, k: usize, ab: [(Usize1, Usize1); n-1], v: [Usize1; k]}
    let mut tree = vec![vec![]; n];
    for (a, b) in ab {
        tree[a].push(b);
        tree[b].push(a);
    }
    let mut vertices: Vec<usize> = Vec::new();
    let mut visited: Vec<bool> = vec![false; n];
    let targets: HashSet<usize> = v.iter().cloned().collect();
    dfs(v[0], &tree, &targets, &mut visited, &mut vertices);
    println!("{}", vertices.len());
}

fn dfs(
    v: usize,
    tree: &Vec<Vec<usize>>,
    targets: &HashSet<usize>,
    visited: &mut Vec<bool>,
    vertices: &mut Vec<usize>,
) -> bool {
    if visited[v] {
        return false;
    }
    visited[v] = true;
    vertices.push(v);
    let mut is_there_target = false;
    if targets.contains(&v) {
        is_there_target = true;
    }
    for &next in &tree[v] {
        if dfs(next, tree, targets, visited, vertices) {
            is_there_target = true;
        }
    }
    if !is_there_target {
        vertices.pop();
    }
    is_there_target
}
