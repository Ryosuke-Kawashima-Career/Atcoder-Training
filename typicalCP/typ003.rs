use proconio::input;
use std::collections::VecDeque;
const INF: usize = usize::MAX;
fn main() {
    input!{n: usize, ab: [(usize, usize); n - 1]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    let dist_from_zero: Vec<usize> = bfs(0, &graph);
    let (start_node, dist1): (usize, usize) = dist_from_zero.iter().enumerate().max_by_key(&(_, d) => *d).unwrap();
    let dist_from_start: Vec<usize> = bfs(start_node, &graph);
    let (_, dist2): (usize, usize) = dist_from_start.iter().enumerate().max_by_key(&(_, d) => *d).unwrap();
    println!("{}", dist1+dist2+1);
}

fn bfs(v: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n: usize = graph.len();
    let mut dist: Vec<usize> = vec![INF; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    dist[v] = 0;
    queue.push_back(v);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if dist[v] == INF {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    dist
}
