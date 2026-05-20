use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
const LIMIT: usize = 5_000_000;
const N1: usize = 1usize.wrapping_neg();
// U, D, L, R
const D4: [(usize, usize); 4] = [(N1, 0), (1, 0), (0, N1), (0, 1)];
fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}
    let (y0, x0, yt, xt) = get_start_goal(&s);
    for dir0 in 0..4 {
        let mut order: Vec<char> = Vec::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
        if dfs(dir0, y0, x0, yt, xt, &s, &mut visited, &mut order) {
            display(&order);
            return;
        }
    }
    println!("No");
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

fn dfs(
    dir0: usize,
    y0: usize,
    x0: usize,
    yt: usize,
    xt: usize,
    graph: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    order: &mut Vec<char>,
) -> bool {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    if visited[y0][x0] {
        return false;
    }
    visited[y0][x0] = true;
    if y0 == yt && x0 == xt {
        return true;
    }
    for dir in 0..4 {
        let (dy, dx) = D4[dir];
        let next_y: usize = y0.wrapping_add(dy);
        let next_x: usize = x0.wrapping_add(dx);
        if next_y < h && next_x < w {
            match graph[next_y][next_x] {
                '#' => {
                    continue;
                }
                '.' | 'S' | 'G' => {
                    if dfs(dir, next_y, next_x, yt, xt, graph, visited, order) {
                        order.push(num_to_dir(dir));
                        return true;
                    } else {
                        continue;
                    }
                }
                'o' => {
                    if dir == dir0 {
                        if dfs(dir, next_y, next_x, yt, xt, graph, visited, order) {
                            order.push(num_to_dir(dir));
                            return true;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                'x' => {
                    if dir != dir0 {
                        if dfs(dir, next_y, next_x, yt, xt, graph, visited, order) {
                            order.push(num_to_dir(dir));
                            return true;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    return false;
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
