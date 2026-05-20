use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Belt {
    cells: Vec<(usize, usize)>,
}

#[derive(Clone)]
struct State {
    grid: Vec<Vec<usize>>,
    box_pos: Vec<(usize, usize)>,
    removed: Vec<bool>,
    next_to_remove: usize,
    operations: Vec<(usize, isize)>,
}

impl State {
    fn apply_shift(
        &mut self,
        belt_id: usize,
        d: isize,
        belts: &[Belt],
        n: usize,
        exit_r: usize,
        exit_c: usize,
    ) {
        let dir = if d > 0 { 1 } else { -1 };
        let steps = d.abs();
        let belt = &belts[belt_id];
        let len = belt.cells.len();

        for _ in 0..steps {
            self.operations.push((belt_id, dir));
            let mut new_vals = vec![0; len];
            for i in 0..len {
                let prev_idx = (i as isize - dir + len as isize) % len as isize;
                let (pr, pc) = belt.cells[prev_idx as usize];
                new_vals[i] = self.grid[pr][pc];
            }
            for i in 0..len {
                let (r, c) = belt.cells[i];
                self.grid[r][c] = new_vals[i];
                if self.grid[r][c] != 999 {
                    self.box_pos[self.grid[r][c]] = (r, c);
                }
            }

            while self.next_to_remove < n * n && self.grid[exit_r][exit_c] == self.next_to_remove {
                self.removed[self.next_to_remove] = true;
                self.grid[exit_r][exit_c] = 999;
                self.next_to_remove += 1;
            }
        }
    }
}

fn get_next_pos(belt: &Belt, r: usize, c: usize, d: isize) -> (usize, usize) {
    let len = belt.cells.len();
    let mut idx = 0;
    for i in 0..len {
        if belt.cells[i] == (r, c) {
            idx = i;
            break;
        }
    }
    let next_idx = (idx as isize + d + len as isize) % len as isize;
    belt.cells[next_idx as usize]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n_line = lines.next();
    if n_line.is_none() {
        return;
    }
    let n: usize = n_line.unwrap().unwrap().trim().parse().unwrap();
    let mut initial_grid = vec![vec![999; n]; n];
    let mut initial_box_pos = vec![(0, 0); n * n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..n {
            initial_grid[i][j] = row[j];
            initial_box_pos[row[j]] = (i, j);
        }
    }

    let exit_r = 0;
    let exit_c = n / 2;

    let mut belts = vec![];
    for k in 0..(n / 2) {
        let mut cells = vec![];
        let r1 = 2 * k;
        let r2 = 2 * k + 1;
        for j in 0..n {
            cells.push((r1, j));
        }
        for j in (0..n).rev() {
            cells.push((r2, j));
        }
        belts.push(Belt { cells });
    }
    for k in 0..(n / 2) {
        let mut cells = vec![];
        let c1 = 2 * k;
        let c2 = 2 * k + 1;
        for i in 0..n {
            cells.push((i, c1));
        }
        for i in (0..n).rev() {
            cells.push((i, c2));
        }
        belts.push(Belt { cells });
    }

    let mut current_state = State {
        grid: initial_grid,
        box_pos: initial_box_pos,
        removed: vec![false; n * n],
        next_to_remove: 0,
        operations: vec![],
    };

    while current_state.next_to_remove < n * n
        && current_state.grid[exit_r][exit_c] == current_state.next_to_remove
    {
        current_state.removed[current_state.next_to_remove] = true;
        current_state.grid[exit_r][exit_c] = 999;
        current_state.next_to_remove += 1;
    }

    for target in 0..(n * n) {
        if current_state.removed[target] {
            continue;
        }

        let (start_r, start_c) = current_state.box_pos[target];

        let mut dist = vec![vec![999usize; n]; n];
        let mut queue = VecDeque::new();
        queue.push_back((start_r, start_c));
        dist[start_r][start_c] = 0;

        while let Some((r, c)) = queue.pop_front() {
            if r == exit_r && c == exit_c {
                break;
            }

            let h_belt_id = r / 2;
            let v_belt_id = 10 + c / 2;

            for &(b_id, d) in &[
                (h_belt_id, 1isize),
                (h_belt_id, -1isize),
                (v_belt_id, 1isize),
                (v_belt_id, -1isize),
            ] {
                let (nr, nc) = get_next_pos(&belts[b_id], r, c, d);
                if dist[nr][nc] == 999 {
                    dist[nr][nc] = dist[r][c] + 1;
                    queue.push_back((nr, nc));
                }
            }
        }

        let mut paths = vec![];
        let mut current_path = vec![];

        fn find_paths(
            r: usize,
            c: usize,
            current_d: usize,
            goal_d: usize,
            dist: &Vec<Vec<usize>>,
            belts: &[Belt],
            current_path: &mut Vec<(usize, isize)>,
            paths: &mut Vec<Vec<(usize, isize)>>,
        ) {
            if paths.len() >= 50 {
                return;
            }
            if r == 0 && c == 10 {
                paths.push(current_path.clone());
                return;
            }
            if current_d >= goal_d {
                return;
            }

            let h_belt_id = r / 2;
            let v_belt_id = 10 + c / 2;
            for &(b_id, d) in &[
                (h_belt_id, 1isize),
                (h_belt_id, -1isize),
                (v_belt_id, 1isize),
                (v_belt_id, -1isize),
            ] {
                let (nr, nc) = get_next_pos(&belts[b_id], r, c, d);
                if dist[nr][nc] == current_d + 1 {
                    current_path.push((b_id, d));
                    find_paths(
                        nr,
                        nc,
                        current_d + 1,
                        goal_d,
                        dist,
                        belts,
                        current_path,
                        paths,
                    );
                    current_path.pop();
                    if paths.len() >= 50 {
                        return;
                    }
                }
            }
        }

        find_paths(
            start_r,
            start_c,
            0,
            dist[exit_r][exit_c],
            &dist,
            &belts,
            &mut current_path,
            &mut paths,
        );

        let mut best_score = f64::MAX;
        let mut best_ops = vec![];

        for ops in paths {
            let mut test_state = current_state.clone();
            for &(b_id, d) in &ops {
                test_state.apply_shift(b_id, d, &belts, n, exit_r, exit_c);
                if test_state.removed[target] {
                    break;
                }
            }

            let mut score = 0.0;
            let lookahead = 10;
            let mut count = 0;
            for j in 0..(n * n) {
                if j >= test_state.next_to_remove && count < lookahead {
                    let (br, bc) = test_state.box_pos[j];
                    let d = (br as isize - exit_r as isize).abs()
                        + (bc as isize - exit_c as isize).abs();
                    score += d as f64;
                    count += 1;
                }
            }
            score += ops.len() as f64 * 0.001;

            if score < best_score {
                best_score = score;
                best_ops = ops;
            }
        }

        for &(b_id, d) in &best_ops {
            current_state.apply_shift(b_id, d, &belts, n, exit_r, exit_c);
            if current_state.removed[target] {
                break;
            }
        }
    }

    println!("{}", belts.len());
    for belt in &belts {
        print!("{}", belt.cells.len());
        for &(r, c) in &belt.cells {
            print!(" {} {}", r, c);
        }
        println!();
    }
    println!("{}", current_state.operations.len());
    for &(id, d) in &current_state.operations {
        println!("{} {}", id, d);
    }
}
