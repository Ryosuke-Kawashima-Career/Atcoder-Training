use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (0, N1), (1, 0), (0, 1)];
const INF: usize = 1 << 60;

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: Usize1,
        cs: Usize1,
        rt: Usize1,
        ct: Usize1,
        s: [Chars; h],
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; h * w];
    for row in 0..h {
        for col in 0..w {
            if s[row][col] == '#' {
                continue;
            }
            let node: usize = get_node(row, col, w);
            for (dir, &(dy, dx)) in D4.iter().enumerate() {
                let next_row: usize = row.wrapping_add(dy);
                let next_col: usize = col.wrapping_add(dx);
                if next_row < h && next_col < w && s[next_row][next_col] == '.' {
                    let next_node: usize = get_node(next_row, next_col, w);
                    graph[node].push((next_node, dir));
                }
            }
        }
    }

    let distance: Vec<[usize; 4]> = bfs01(&graph, get_node(rs, cs, w));
    let ans: usize = *distance[get_node(rt, ct, w)].iter().min().unwrap();
    println!("{}", ans);
}

fn bfs01(graph: &Vec<Vec<(usize, usize)>>, start_node: usize) -> Vec<[usize; 4]> {
    let n: usize = graph.len();
    let mut que = VecDeque::new();
    let mut dist: Vec<[usize; 4]> = vec![[INF; 4]; n];

    for dir in 0..4 {
        dist[start_node][dir] = 0;
        que.push_back((start_node, dir));
    }

    while let Some((curr_node, curr_dir)) = que.pop_front() {
        let curr_dist = dist[curr_node][curr_dir];
        for &(next_node, next_dir) in graph[curr_node].iter() {
            let cost = if curr_dir == next_dir { 0 } else { 1 };
            let next_dist = curr_dist + cost;
            if next_dist < dist[next_node][next_dir] {
                dist[next_node][next_dir] = next_dist;
                if cost == 0 {
                    que.push_front((next_node, next_dir));
                } else {
                    que.push_back((next_node, next_dir));
                }
            }
        }
    }

    dist
}

fn get_node(row: usize, col: usize, w: usize) -> usize {
    row * w + col
}
