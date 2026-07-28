use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
// count the number of nodes reachable from node 0
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(from, to) in ab.iter() {
        graph[from].push(to);
    }

    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    visited[0] = true;
    queue.push_back(0);
    let mut count = 0;

    while let Some(v) = queue.pop_front() {
        count += 1;
        for &next in &graph[v] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    println!("{}", count);
}
