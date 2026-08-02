use proconio::{input, marker::Chars};
use std::collections::VecDeque;

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // 0: U, 1: D, 2: L, 3: R
const DIR_CHARS: [char; 4] = ['U', 'D', 'L', 'R'];
const INF: usize = usize::MAX / 2;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut sr = 0;
    let mut sc = 0;
    let mut gr = 0;
    let mut gc = 0;

    for r in 0..h {
        for c in 0..w {
            if s[r][c] == 'S' {
                sr = r;
                sc = c;
            } else if s[r][c] == 'G' {
                gr = r;
                gc = c;
            }
        }
    }

    let mut dist = vec![vec![vec![INF; 4]; w]; h];
    let mut parent = vec![vec![vec![(0, 0, 4); 4]; w]; h]; // pd = 4 means starting node
    let mut que = VecDeque::new();

    // From S, Takahashi can move in any direction
    for d in 0..4 {
        let nr = sr as isize + DIRS[d].0;
        let nc = sc as isize + DIRS[d].1;
        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
            let nr = nr as usize;
            let nc = nc as usize;
            if s[nr][nc] != '#' {
                dist[nr][nc][d] = 1;
                parent[nr][nc][d] = (sr, sc, 4);
                que.push_back((nr, nc, d));
            }
        }
    }

    while let Some((r, c, d)) = que.pop_front() {
        let cell = s[r][c];
        for next_d in 0..4 {
            if cell == 'o' && next_d != d {
                continue;
            }
            if cell == 'x' && next_d == d {
                continue;
            }

            let nr = r as isize + DIRS[next_d].0;
            let nc = c as isize + DIRS[next_d].1;

            if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
                let nr = nr as usize;
                let nc = nc as usize;
                if s[nr][nc] != '#' && dist[nr][nc][next_d] == INF {
                    dist[nr][nc][next_d] = dist[r][c][d] + 1;
                    parent[nr][nc][next_d] = (r, c, d);
                    que.push_back((nr, nc, next_d));
                }
            }
        }
    }

    let mut min_d = 4;
    let mut min_dist = INF;
    for d in 0..4 {
        if dist[gr][gc][d] < min_dist {
            min_dist = dist[gr][gc][d];
            min_d = d;
        }
    }

    if min_dist == INF {
        println!("No");
    } else {
        println!("Yes");
        let mut path = Vec::new();
        let mut cur_r = gr;
        let mut cur_c = gc;
        let mut cur_d = min_d;

        while cur_d != 4 {
            path.push(DIR_CHARS[cur_d]);
            let (pr, pc, pd) = parent[cur_r][cur_c][cur_d];
            cur_r = pr;
            cur_c = pc;
            cur_d = pd;
        }

        path.reverse();
        let path_str: String = path.into_iter().collect();
        println!("{}", path_str);
    }
}
