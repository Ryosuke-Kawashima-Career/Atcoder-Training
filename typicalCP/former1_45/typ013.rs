use proconio::{input, marker::Usize1};
const INF: i64 = i64::MAX;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
fn main() {
    input! {n: usize, m: usize, abc: [(Usize1, Usize1, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(a, b, c) in abc.iter() {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }
    let dist_from_start: Vec<i64> = dijkstra(0, &graph);
    let dist_from_end: Vec<i64> = dijkstra(n - 1, &graph);
    for k in 0..n {
        let distance: i64 = dist_from_start[k] + dist_from_end[k];
        println!("{}", distance);
    }
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![INF; n];
    let mut min_que = BinaryHeap::new();
    min_que.push((Reverse(0), start));
    dist[start] = 0;
    while let Some(Reverse(min_dist), curr_v) = min_que.pop() {
        for &(next, cost) in graph[curr_v].iter() {
            if dist[next] > min_dist + cost {
                dist[next] = min_dist + cost;
                min_que.push((Reverse(dist[next]), next));
            }
        }
    }
    dist
}
