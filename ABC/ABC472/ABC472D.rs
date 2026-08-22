use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
fn main() {
    input!{h: usize, w: usize, k: usize, s: [Chars; h]}
    let (dangerous_rows, dangerous_cols)= get_dangerous(&s);
    let dist: Vec<Vec<usize>> = bfs(&dangerous_rows, &dangerous_cols, &s);
    let mut count: usize = 0;
    for i in 0..h {
        for j in 0..w {
            if dist[i][j] <= k && s[i][j] == '.' {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn get_dangerous(graph: &Vec<Vec<char>>) -> (Vec<bool>, Vec<bool>) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dangerous_rows: Vec<bool> = vec![false; h];
    let mut dangerous_cols: Vec<bool> = vec![false; w];
    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == '#' {
                dangerous_rows[i] = true;
                dangerous_cols[j] = true;
            }
        }
    }
    (dangerous_rows, dangerous_cols)
}

fn bfs(dangerous_rows: &Vec<bool>, dangerous_cols: &Vec<bool>, graph: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    let mut que = std::collections::VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if !dangerous_rows[i] && !dangerous_cols[j] && graph[i][j] == '.' {
                dist[i][j] = 0;
                que.push_back((i, j));
            }
        }
    }

    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y.wrapping_add(dy);
            let next_x: usize = x.wrapping_add(dx);
            if next_y < h && next_x < w && graph[next_y][next_x] == '.' {
                if dist[next_y][next_x] > dist[y][x] + 1 {
                    dist[next_y][next_x] = dist[y][x] + 1;
                    que.push_back((next_y, next_x));
                }
            }
        }
    }

    dist
}
