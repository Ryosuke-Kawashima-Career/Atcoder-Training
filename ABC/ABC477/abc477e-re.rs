use proconio::input;
use proconio::marker::Usize1;
fn main() {
    input! {n: usize, q: usize, a: [i64; n], b: [i64; n], queries: [(Usize1, Usize1); q]}
    let mut prefix_dist: Vec<i64> = vec![0; n + 1];
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n + 1];
    for i in 0..n {
        prefix_dist[i + 1] = prefix_dist[i] + a[i];
        graph[i].push(((i + 1) % n, a[i]));
        graph[(i + 1) % n].push((i, b[i]));
        graph[i].push((n, b[i]));
        graph[n].push((i, b[i]));
    }
    let dist_from_center: Vec<i64> = dijkstra(&graph, n);
    for &(start, goal) in queries.iter() {
        let candidate1: i64 = prefix_dist[goal] - prefix_dist[start];
        let candidate2: i64 = prefix_dist[n] - candidate1;
        let center_dist: i64 = dist_from_center[start] + dist_from_center[goal];
        let min_dist: i64 = candidate1.min(candidate2).min(center_dist);
        println!("{}", min_dist);
    }
}

fn dijkstra(graph: &Vec<Vec<(usize, i64)>>, center: usize) -> Vec<i64> {
    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![i64::MAX; n];
    dist[center] = 0;
    let mut heap: std::collections::BinaryHeap<std::cmp::Reverse<(i64, usize)>> =
        std::collections::BinaryHeap::new();
    heap.push(std::cmp::Reverse((0, center)));
    while let Some(std::cmp::Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in graph[u].iter() {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                heap.push(std::cmp::Reverse((dist[v], v)));
            }
        }
    }
    dist
}
