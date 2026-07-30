use proconio::{input, marker::Chars};
use std::collections::VecDeque;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (1, 0), (0, N1), (0, 1)];
const INF: usize = usize::MAX / 2;
fn num_to_dir(num: usize) -> char {
    match num {
        0 => 'D',
        1 => 'U',
        2 => 'R',
        3 => 'L',
        _ => panic!("Invalid direction number"),
    }
}
fn get_start_end(graph: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut start_row: usize = 0;
    let mut start_col: usize = 0;
    let mut end_row: usize = 0;
    let mut end_col: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == 'S' {
                start_row = i;
                start_col = j;
            } else if graph[i][j] == 'G' {
                end_row = i;
                end_col = j;
            }
        }
    }
    (start_row, start_col, end_row, end_col)
}
fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}
    let (start_row, start_col, end_row, end_col) = get_start_end(&s);
    let dist: Vec<Vec<Vec<usize>>> = bfs(&s, start_row, start_col);
    let mut min_dist: usize = INF;
    for dir in 0..4 {
        min_dist = min_dist.min(dist[end_row][end_col][dir]);
    }
    if min_dist == INF {
        println!("No");
    } else {
        println!("Yes");
        let path: Vec<char> = get_path(&dist, start_row, start_col, end_row, end_col);
        for s in path {
            print!("{}", s);
        }
        println!("");
    }
}

fn get_path(
    dist: &Vec<Vec<Vec<usize>>>,
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
) -> Vec<char> {
    let mut min_dist: usize = INF;
    let mut min_dir: usize = 4;
    for dir in 0..4 {
        if min_dist > dist[end_row][end_col][dir] {
            min_dist = dist[end_row][end_col][dir];
            min_dir = dir;
        }
    }
    let mut path: Vec<char> = Vec::new();
    let mut cur_row: usize = end_row;
    let mut cur_col: usize = end_col;
    let mut cur_dir: usize = min_dir;
    while cur_row != start_row || cur_col != start_col {
        path.push(num_to_dir(cur_dir));
        let (dr, dc) = D4[cur_dir];
        cur_row = cur_row.wrapping_sub(dr);
        cur_col = cur_col.wrapping_sub(dc);
    }
    path.reverse();
    return path;
}

fn bfs(graph: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> Vec<Vec<Vec<usize>>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; 4]; w]; h];
    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();
    for dir in 0..4 {
        dist[start_row][start_col][dir] = 0;
        que.push_back((start_row, start_col, dir));
    }
    while let Some((cur_row, cur_col, cur_dir)) = que.pop_front() {
        let (dr, dc) = D4[cur_dir];
        for dir in 0..4 {
            let next_row = cur_row.wrapping_add(dr);
            let next_col = cur_col.wrapping_add(dc);
            if (next_row < 0 || next_row >= h || next_col < 0 || next_col >= w) {
                continue;
            }
            if graph[next_row][next_col] == '#' {
                continue;
            } else if graph[next_row][next_col] == 'o' {
                if dir == cur_dir && dist[next_row][next_col][dir] > dist[cur_row][cur_col][cur_dir]
                {
                    dist[next_row][next_col][dir] = dist[cur_row][cur_col][cur_dir];
                    que.push_back((next_row, next_col, dir));
                }
            } else if graph[next_row][next_col] == 'x' {
                if dir != cur_dir
                    && dist[next_row][next_col][dir] > dist[cur_row][cur_col][cur_dir] + 1
                {
                    dist[next_row][next_col][dir] = dist[cur_row][cur_col][cur_dir] + 1;
                    que.push_back((next_row, next_col, dir));
                }
            } else {
                if dist[next_row][next_col][dir] > dist[cur_row][cur_col][cur_dir] + 1 {
                    dist[next_row][next_col][dir] = dist[cur_row][cur_col][cur_dir] + 1;
                    que.push_back((next_row, next_col, dir));
                }
            }
        }
    }
    return dist;
}
