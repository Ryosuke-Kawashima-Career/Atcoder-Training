use proconio::{input, marker::Chars};
use std::collections::VecDeque;

const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(0, 1), (0, N1), (1, 0), (N1, 0)];

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut ans: usize = 0;
    for y in 0..h {
        for x in 0..w {
            if s[y][x] == '.' && !visited[y][x] {
                if bfs(y, x, &s, &mut visited) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

fn bfs(y: usize, x: usize, s: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> bool {
    let h = s.len();
    let w = s[0].len();
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((y, x));
    visited[y][x] = true;
    let mut is_ok: bool = true;
    while let Some((y, x)) = q.pop_front() {
        for (dy, dx) in D4 {
            let ny = y.wrapping_add(dy);
            let nx = x.wrapping_add(dx);
            if ny < h && nx < w {
                if s[ny][nx] == '.' && !visited[ny][nx] {
                    visited[ny][nx] = true;
                    q.push_back((ny, nx));
                }
            } else {
                is_ok = false;
            }
        }
    }
    is_ok
}
