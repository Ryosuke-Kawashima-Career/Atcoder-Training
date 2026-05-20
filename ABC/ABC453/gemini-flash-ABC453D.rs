use proconio::{input, marker::Chars};
use std::collections::VecDeque;

const LIMIT: usize = 5_000_000;
const N1: usize = 1usize.wrapping_neg();
// Directions: 0: Up, 1: Down, 2: Left, 3: Right
const D4: [(usize, usize); 4] = [(N1, 0), (1, 0), (0, N1), (0, 1)];
// ABC453D
// Q. Reachable from the start to the end: o = the same direction x = different directions
// A. DFS with pruning to ensure you do not go on the same path.
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h]
    }

    let (y0, x0, yt, xt) = get_start_goal(&s);
    let (prev_cells, reachable) = bfs_reachable(y0, x0, yt, xt, &s);

    if reachable {
        let path = recon_path(&prev_cells, y0, x0, yt, xt);
        display(&path);
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
    let h = graph.len();
    let w = graph[0].len();

    // prev_cells[y][x][dir] stores (prev_y, prev_x, prev_dir_in)
    // dir is the direction used to ENTER (y, x).
    // Sentinel (h, w, 4) means unvisited.
    let mut prev_cells = vec![vec![vec![(h, w, 4); 4]; w]; h];

    if y0 == yt && x0 == xt {
        return (prev_cells, true);
    }

    let mut que = VecDeque::new();

    // Initial moves from S. S has no preceding direction, represented by dummy 4.
    // However, since we store dir = 0..3 in prev_cells, we bootstrap by pushing
    // the first valid moves out of S.
    for dir in 0..4 {
        let (dy, dx) = D4[dir];
        let ny = y0.wrapping_add(dy);
        let nx = x0.wrapping_add(dx);
        if ny < h && nx < w && graph[ny][nx] != '#' {
            if prev_cells[ny][nx][dir] == (h, w, 4) {
                prev_cells[ny][nx][dir] = (y0, x0, 4); // 4 means came from start
                que.push_back((ny, nx, dir));
            }
        }
    }

    while let Some((y, x, dir)) = que.pop_front() {
        if y == yt && x == xt {
            return (prev_cells, true);
        }

        // Apply rules of the CURRENT cell (y, x) to decide the next move.
        // dir is the direction used to enter (y, x).
        let c = graph[y][x];

        let allowed_dirs = match c {
            'o' => vec![dir], // Must continue in the same direction
            'x' => {
                // Must change direction (180 deg turn is allowed as a change)
                let mut v = Vec::with_capacity(3);
                for d in 0..4 {
                    if d != dir {
                        v.push(d);
                    }
                }
                v
            }
            _ => vec![0, 1, 2, 3], // '.', 'S', 'G' allow any direction
        };

        for next_dir in allowed_dirs {
            let (dy, dx) = D4[next_dir];
            let ny = y.wrapping_add(dy);
            let nx = x.wrapping_add(dx);

            if ny < h && nx < w && graph[ny][nx] != '#' {
                if prev_cells[ny][nx][next_dir] == (h, w, 4) {
                    prev_cells[ny][nx][next_dir] = (y, x, dir);
                    que.push_back((ny, nx, next_dir));
                }
            }
        }
    }

    (prev_cells, false)
}

fn recon_path(
    prev_cells: &Vec<Vec<Vec<(usize, usize, usize)>>>,
    y0: usize,
    x0: usize,
    yt: usize,
    xt: usize,
) -> Vec<char> {
    let h = prev_cells.len();
    let w = prev_cells[0].len();

    // Find any direction that reached the goal
    for start_dir in 0..4 {
        if prev_cells[yt][xt][start_dir] == (h, w, 4) {
            continue;
        }

        let mut path = Vec::new();
        let mut curr_y = yt;
        let mut curr_x = xt;
        let mut curr_dir = start_dir;

        while curr_y != y0 || curr_x != x0 {
            path.push(num_to_dir(curr_dir));
            let (py, px, pdir) = prev_cells[curr_y][curr_x][curr_dir];
            curr_y = py;
            curr_x = px;
            curr_dir = pdir;
        }

        path.reverse();
        return path;
    }
    vec![]
}

fn get_start_goal(s: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for (y, row) in s.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                start = (y, x);
            }
            if c == 'G' {
                goal = (y, x);
            }
        }
    }
    (start.0, start.1, goal.0, goal.1)
}

fn num_to_dir(num: usize) -> char {
    match num {
        0 => 'U',
        1 => 'D',
        2 => 'L',
        3 => 'R',
        _ => unreachable!(),
    }
}

fn display(path: &Vec<char>) {
    if path.len() <= LIMIT {
        println!("Yes");
        let s: String = path.iter().collect();
        println!("{}", s);
    } else {
        println!("No");
    }
}
