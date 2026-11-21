use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
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

fn bfs(start: usize, graph: &[Vec<(usize, usize)>]) -> Option<usize> {
    let n: usize = graph.len();
    let mut vis_count: Vec<u8> = vec![0; n];
    let mut first_color: Vec<usize> = vec![usize::MAX; n];
    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();

    vis_count[start] = 2;
    que.push_back((start, usize::MAX, 0));

    while let Some((curr, prev_color, dist)) = que.pop_front() {
        if curr == n - 1 {
            return Some(dist);
        }
        for &(next, color) in graph[curr].iter() {
            if color != prev_color {
                if vis_count[next] == 0 {
                    vis_count[next] = 1;
                    first_color[next] = color;
                    que.push_back((next, color, dist + 1));
                } else if vis_count[next] == 1 && first_color[next] != color {
                    vis_count[next] = 2;
                    que.push_back((next, color, dist + 1));
                }
            }
        }
    }
    None
}
