use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;
// ABC395E
// Q. Find the shortest path from node 0 to node n-1 when the flipping the direction of the edges costs x.
// A. Dijkstra's algorithm
fn main() {
    input! {n: usize, m: usize, x: i64, edges: [(Usize1, Usize1); m]}
    // graph[direction][from][to]
    let mut graph: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; n]; 2];
    for (a, b) in edges {
        graph[0][a].push(b);
        graph[1][b].push(a);
    }
    // dist[direction][node]
    let dist = dijkstra(0, &graph, x);
    let ans = dist[0][n - 1].min(dist[1][n - 1]);
    println!("{}", ans);
}

fn dijkstra(start: usize, graph: &Vec<Vec<Vec<usize>>>, x: i64) -> Vec<Vec<i64>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n: usize = graph[0].len();
    // dist[direction][node]
    let mut dist: Vec<Vec<i64>> = vec![vec![INF; n]; 2];
    // (min_distance, node, direction)
    let mut min_dist = BinaryHeap::new();
    dist[0][start] = 0;
    min_dist.push((Reverse(0), start, 0));
    while let Some((Reverse(distance), cur_v, cur_dir)) = min_dist.pop() {
        if distance > dist[cur_dir][cur_v] {
            continue;
        }
        // does not change the direction of the edges
        for &next_v in &graph[cur_dir][cur_v] {
            let next_distance = distance + 1;
            if next_distance < dist[cur_dir][next_v] {
                dist[cur_dir][next_v] = next_distance;
                min_dist.push((Reverse(next_distance), next_v, cur_dir));
            }
        }
        // changes the direction of the edges
        for &next_v in &graph[1 - cur_dir][cur_v] {
            let next_dir = 1 - cur_dir;
            let next_distance = distance + x;
            if next_distance < dist[next_dir][cur_v] {
                dist[next_dir][cur_v] = next_distance;
                // you can stay at the current node!!!
                min_dist.push((Reverse(next_distance), cur_v, next_dir));
            }
        }
    }
    dist
}
