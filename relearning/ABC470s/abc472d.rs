use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashSet, VecDeque};
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(0, N1), (N1, 0), (0, 1), (1, 0)];
fn main() {
    input! {h: usize, w: usize, k: usize, s: [Chars; h]}
    let (dangerous_rows, dangerous_cols) = get_dangers(&s);
    let distance: Vec<Vec<usize>> = bfs(&dangerous_rows, &dangerous_cols, &s);
    let mut ans: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if distance[i][j] <= k && s[i][j] == '.' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn bfs(
    dangerous_rows: &HashSet<usize>,
    dangerous_cols: &HashSet<usize>,
    s: &Vec<Vec<char>>,
) -> Vec<Vec<usize>> {
    let h: usize = s.len();
    let w: usize = s[0].len();
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; w]; h];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if !dangerous_rows.contains(&i) && !dangerous_cols.contains(&j) {
                que.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }
    while let Some(curr) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = curr.0.wrapping_add(dy);
            let next_x: usize = curr.1.wrapping_add(dx);
            if next_x < w
                && next_y < h
                && dist[next_y][next_x] > dist[curr.0][curr.1] + 1
                && s[next_y][next_x] == '.'
            {
                dist[next_y][next_x] = dist[curr.0][curr.1] + 1;
                que.push_back((next_y, next_x));
            }
        }
    }
    dist
}

fn get_dangers(s: &Vec<Vec<char>>) -> (HashSet<usize>, HashSet<usize>) {
    let h: usize = s.len();
    let w: usize = s[0].len();
    let mut dangerous_rows = HashSet::new();
    let mut dangerous_cols = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                dangerous_rows.insert(i);
                dangerous_cols.insert(j);
            }
        }
    }
    (dangerous_rows, dangerous_cols)
}
