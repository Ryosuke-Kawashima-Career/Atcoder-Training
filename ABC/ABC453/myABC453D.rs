use proconio::{input, marker::Chars};
use std::collections::VecDeque;
const INF: usize = 1 << 60;
const LIMIT: usize = 5_000_000;
const N1: usize = 1usize.wrapping_neg();
// U, D, L, R
const D4: [(usize, usize); 4] = [(N1, 0), (1, 0), (0, N1), (0, 1)];
// ABC453D
// Q. Reachable from the start to the end: o = the same direction x = different directions
// A. DFS with pruning to ensure you do not go on the same path.
fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}
    let (y0, x0, yt, xt) = get_start_goal(&s);
    let (prev_cells, reachable) = bfs_reachable(y0, x0, yt, xt, &s);
    if reachable {
        let path: Vec<char> = recon_path(&prev_cells, y0, x0, yt, xt);
        display(&path);
    } else {
        println!("No");
    }
}

fn display(order: &Vec<char>) {
    let n: usize = order.len();
    if n <= LIMIT {
        println!("Yes");
        for i in 0..n {
            print!("{}", order[i]);
        }
        println!("");
    } else {
        println!("No");
    }
}

fn bfs_reachable(
    y0: usize,
    x0: usize,
    yt: usize,
    xt: usize,
    graph: &Vec<Vec<char>>,
) -> (Vec<Vec<Vec<(usize, usize, usize)>>>, bool) {
    // Multi-dimensional vertex(頂点倍化): (Y axis, X axis, Direction)
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut prev_cells: Vec<Vec<Vec<(usize, usize, usize)>>> = vec![vec![vec![(h, w, 4); 4]; w]; h];
    if y0 == yt && x0 == xt {
        return (prev_cells, true);
    }
    let mut que: VecDeque<(usize, usize, usize)> = VecDeque::new();
    for dir in 0..4 {
        que.push_back((y0, x0, dir));
    }

    while let Some((y, x, dir)) = que.pop_front() {
        if y == yt && x == xt {
            return (prev_cells, true);
        }
        for next_dir in 0..4 {
            let (dy, dx) = D4[next_dir];
            let next_y: usize = y.wrapping_add(dy);
            let next_x: usize = x.wrapping_add(dx);
            if next_y < h && next_x < w {
                // if visited continue
                if prev_cells[next_y][next_x][next_dir] != (h, w, 4) {
                    continue;
                }
                match graph[next_y][next_x] {
                    '#' => {
                        continue;
                    }
                    '.' | 'S' | 'G' => {
                        prev_cells[next_y][next_x][next_dir] = (y, x, dir);
                        que.push_back((next_y, next_x, next_dir));
                    }
                    'o' => {
                        if dir == next_dir {
                            prev_cells[next_y][next_x][next_dir] = (y, x, dir);
                            que.push_back((next_y, next_x, next_dir));
                        }
                    }
                    'x' => {
                        if dir != next_dir {
                            prev_cells[next_y][next_x][next_dir] = (y, x, dir);
                            que.push_back((next_y, next_x, next_dir));
                        }
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }

    return (prev_cells, false);
}

fn recon_path(
    prev_cells: &Vec<Vec<Vec<(usize, usize, usize)>>>,
    y0: usize,
    x0: usize,
    yt: usize,
    xt: usize,
) -> Vec<char> {
    let h: usize = prev_cells.len();
    let w: usize = prev_cells[0].len();
    for mut curr_dir in 0..4 {
        let mut path: Vec<char> = Vec::new();
        let mut curr_y: usize = yt;
        let mut curr_x: usize = xt;
        if prev_cells[yt][xt][curr_dir] == (h, w, 4) {
            continue;
        }
        while curr_y != y0 || curr_x != x0 {
            let (prev_y, prev_x, prev_dir) = prev_cells[curr_y][curr_x][curr_dir];
            path.push(num_to_dir(curr_dir));
            curr_y = prev_y;
            curr_x = prev_x;
            curr_dir = prev_dir;
        }
        path.reverse();
        return path;
    }
    return vec![];
}

fn get_start_goal(s: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let h = s.len();
    let w = s[0].len();
    let mut y0 = 0;
    let mut x0 = 0;
    let mut yt = 0;
    let mut xt = 0;
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == 'S' {
                y0 = y;
                x0 = x;
            }
            if s[y][x] == 'G' {
                yt = y;
                xt = x;
            }
        }
    }
    (y0, x0, yt, xt)
}

fn num_to_dir(num: usize) -> char {
    match num {
        0 => 'U',
        1 => 'D',
        2 => 'L',
        3 => 'R',
        _ => {
            unreachable!();
        }
    }
}
