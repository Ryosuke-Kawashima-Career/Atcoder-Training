use proconio::{input, marker::Usize1};
use std::collections::BinaryHeap;
fn main() {
    input! {n: usize, m: usize, y: i64, uvt: [(Usize1, Usize1, i64); m], x: [i64; n]};
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(u, v, time) in uvt.iter() {
        graph[u].push((v, time));
        graph[v].push((u, time));
    }
    let dist: Vec<i64> = dijkstra_wrap(&graph, y, &x);
    for k in 1..n {
        print!("{} ", dist[k]);
    }
    println!("");
}

fn dijkstra_wrap(graph: &Vec<Vec<(usize, i64)>>, y: i64, x: &Vec<i64>) -> Vec<i64> {
    let n = graph.len();
    let mut dist: Vec<i64> = vec![i64::MAX; n];
    // queue<distance, cur_node_index>
    let mut queue = BinaryHeap::new();
    dist[0] = 0;
    queue.push((0, 0));
    while let Some((cur_min_dist, cur_node)) = queue.pop() {
        if cur_min_dist > dist[cur_node] {
            continue;
        }
        for &(next_node, time) in graph[cur_node].iter() {
            if dist[next_node] > dist[cur_node] + time {
                dist[next_node] = dist[cur_node] + time;
                queue.push((dist[next_node], next_node));
            }
        }
        for next_node in 0..n {
            let time: i64 = x[cur_node] + x[next_node] + y;
            if dist[next_node] > dist[cur_node] + time {
                dist[next_node] = dist[cur_node] + time;
                queue.push((dist[next_node], next_node));
            }
        }
    }
    return dist;
}
