use proconio::{input, marker::Chars};
use std::collections::VecDeque;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (1, 0), (0, N1), (0, 1)];
const INF: usize = usize::MAX / 2;
fn num_to_dir(num: usize) -> char {
    match num {
        0 => 'U',
        1 => 'D',
        2 => 'L',
        3 => 'R',
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
    let (dist, parent) = bfs(&s, start_row, start_col);
    let mut min_dist: usize = INF;
    for dir in 0..4 {
        min_dist = min_dist.min(dist[end_row][end_col][dir]);
    }
    if min_dist == INF {
        println!("No");
    } else {
        println!("Yes");
        let path: Vec<char> = get_path(&dist, &parent, start_row, start_col, end_row, end_col);
        for s in path {
            print!("{}", s);
        }
        println!("");
    }
}

fn get_path(
    dist: &Vec<Vec<Vec<usize>>>,
    parent: &Vec<Vec<Vec<(usize, usize, usize)>>>,
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
        let (parent_row, parent_col, parent_dir) = parent[cur_row][cur_col][cur_dir];
        cur_row = parent_row;
        cur_col = parent_col;
        cur_dir = parent_dir;
    }
    path.reverse();
    return path;
}

fn bfs(
    graph: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
) -> (Vec<Vec<Vec<usize>>>, Vec<Vec<Vec<(usize, usize, usize)>>>) {
    /*
    Returns:
        (dist, parents)
        dist[row][col][dir] is the minimum distance to reach (row, col) with final direction dir
        parents[row][col][dir] is the parent of (row, col) with final direction dir, represented as (parent_row, parent_col, parent_dir)
        where dir=4 means parent is the starting node
     */
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; 4]; w]; h];
    dist[start_row][start_col] = vec![0; 4];
    let mut parent: Vec<Vec<Vec<(usize, usize, usize)>>> = vec![vec![vec![(0, 0, 4); 4]; w]; h];
    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();
    for dir in 0..4 {
        let next_row: usize = start_row.wrapping_add(D4[dir].0);
        let next_col: usize = start_col.wrapping_add(D4[dir].1);
        if next_row >= h || next_col >= w {
            continue;
        }
        if graph[next_row][next_col] != '#' {
            dist[next_row][next_col][dir] = 1;
            parent[next_row][next_col][dir] = (start_row, start_col, 4);
            que.push_back((next_row, next_col, dir));
        }
    }
    while let Some((cur_row, cur_col, cur_dir)) = que.pop_front() {
        for dir in 0..4 {
            if graph[cur_row][cur_col] == 'o' && dir != cur_dir {
                continue;
            }
            if graph[cur_row][cur_col] == 'x' && dir == cur_dir {
                continue;
            }
            let (dr, dc) = D4[dir];
            let next_row = cur_row.wrapping_add(dr);
            let next_col = cur_col.wrapping_add(dc);
            if next_row >= h || next_col >= w {
                continue;
            }
            if graph[next_row][next_col] == '#' {
                continue;
            }
            if dist[next_row][next_col][dir] > dist[cur_row][cur_col][cur_dir] + 1 {
                dist[next_row][next_col][dir] = dist[cur_row][cur_col][cur_dir] + 1;
                parent[next_row][next_col][dir] = (cur_row, cur_col, cur_dir);
                que.push_back((next_row, next_col, dir));
            }
        }
    }
    return (dist, parent);
}
