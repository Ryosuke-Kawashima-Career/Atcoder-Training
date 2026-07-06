use proconio::{input, marker::Chars};
use std::collections::VecDeque;
const INF: usize = usize::MAX;
const N1: usize = 1usize.wrapping_neg();
const D8: [(usize, usize); 8] = [
    (N1, N1),
    (0, N1),
    (1, N1),
    (N1, 0),
    (1, 0),
    (N1, 1),
    (0, 1),
    (1, 1),
];
fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}
    let mut black_cells: Vec<(usize, usize)> = Vec::new();
    let mut white_cells: Vec<(usize, usize)> = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                black_cells.push((i, j));
            } else {
                white_cells.push((i, j));
            }
        }
    }
    let dist_black = bfs_multistarts(&s, &black_cells);
    let dist_white = bfs_multistarts(&s, &white_cells);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                if dist_white[i][j] != INF && dist_white[i][j] % 2 == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                if dist_black[i][j] != INF && dist_black[i][j] % 2 == 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

fn bfs_multistarts(graph: &Vec<Vec<char>>, starts: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    /* Calculates distances from the points of the same color(white or black)
    Args:
        starts: Points of black or white
    returns:
        vector of distances from the starts
     */
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();
    for &start in starts.iter() {
        que.push_back(start);
        dist[start.0][start.1] = 0;
    }
    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in D8.iter() {
            let ny: isize = y as isize + dy as isize;
            let nx: isize = x as isize + dx as isize;
            if ny < 0 || nx < 0 || ny >= h as isize || nx >= w as isize {
                continue;
            }
            let ny: usize = ny as usize;
            let nx: usize = nx as usize;
            if dist[ny][nx] == INF {
                dist[ny][nx] = dist[y][x] + 1;
                que.push_back((ny, nx));
            }
        }
    }
    dist
}
