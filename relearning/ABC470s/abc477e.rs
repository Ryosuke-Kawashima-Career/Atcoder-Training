use proconio::input;
use std::collections::BinaryHeap;
fn main() {
    input! {n: usize, q: usize, a: [i64; n], b: [i64; n]}
    let sum_a: i64 = a.iter().sum();
    let mut prefix: Vec<i64> = vec![0; n + 2];
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
        graph[i].push((i + 1, a[i]));
        graph[i + 1].push((i, a[i]));
        graph[i].push((n, b[i]));
        graph[n].push((i, b[i]));
    }
    let dist_from_n: Vec<i64> = dijkstra(n, &graph);
    for _query in 0..q {
        input! {s: usize, t: usize}
        let candidate1: i64 = (prefix[t - 1] - prefix[s - 1]).abs();
        let candidate2: i64 = sum_a - candidate1;
        let candidate3: i64 = dist_from_n[s - 1] + dist_from_n[t - 1];
        let ans: i64 = candidate1.min(candidate2).min(candidate3);
        println!("{}", ans);
    }
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![i64::MAX; n];
    let mut pq: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    dist[start] = 0;
    pq.push((0, start));
    while let Some((d, v)) = pq.pop() {
        if d > dist[v] {
            continue;
        }
        for (neighbor, weight) in graph[v].iter() {
            let new_dist = dist[v] + weight;
            if new_dist < dist[*neighbor] {
                dist[*neighbor] = new_dist;
                pq.push((new_dist, *neighbor));
            }
        }
    }
    dist
}
