use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
// abc485E
// Q. Given a graph with colored edges. You cannot traverse the paths of the same color twice. What is the shortest path from 0 to n-1.
// A. Breadth-First Search (BFS) with Top-2 Incoming Colors per Vertex
fn main() {
    input! {
        n: usize,
        m: usize,
        uvc: [(Usize1, Usize1, Usize1); m],
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for &(u, v, c) in uvc.iter() {
        graph[u].push((v, c));
    }

    match bfs(0, &graph) {
        Some(min_dist) => println!("{}", min_dist),
        None => println!("-1"),
    }
}

fn bfs(start: usize, graph: &Vec<Vec<(usize, usize)>>) -> Option<usize> {
    let n: usize = graph.len();
    let mut shortest_distance: Vec<usize> = vec![usize::MAX; n];
    let mut optimal_color: Vec<Option<usize>> = vec![None; n];
    let mut non_optimal_color_distance: Vec<usize> = vec![usize::MAX; n];
    // Queue<(curr_node, color, distance)>
    let mut que = VecDeque::new();
    shortest_distance[start] = 0;
    non_optimal_color_distance[start] = 0;
    que.push_back((start, None, 0));
    while let Some((curr, curr_color, curr_distance)) = que.pop_front() {
        for &(next, next_color) in graph[curr].iter() {
            if curr_color == Some(next_color) {
                continue;
            }
            if shortest_distance[next] == usize::MAX {
                shortest_distance[next] = curr_distance + 1;
                optimal_color[next] = Some(next_color);
                que.push_back((next, Some(next_color), shortest_distance[next]));
            } else if optimal_color[next] != Some(next_color)
                && non_optimal_color_distance[next] == usize::MAX
            {
                non_optimal_color_distance[next] = curr_distance + 1;
                que.push_back((next, Some(next_color), non_optimal_color_distance[next]));
            }
        }
    }
    if shortest_distance[n-1] == usize::MAX {
        return None;
    }
    return Some(shortest_distance[n-1]);
}
