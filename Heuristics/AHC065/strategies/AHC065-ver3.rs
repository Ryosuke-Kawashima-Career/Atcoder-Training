use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Belt {
    cells: Vec<(usize, usize)>,
}

#[derive(Clone)]
struct State {
    grid: Vec<u16>,
    box_pos: Vec<(u8, u8)>,
    next_to_remove: usize,
}

impl State {
    fn apply_shift(&mut self, belt_id: usize, d: isize, belts: &[Belt]) {
        let dir = if d > 0 { 1 } else { -1 };
        let steps = d.abs();
        let belt = &belts[belt_id];
        let len = belt.cells.len();

        for _ in 0..steps {
            let mut new_vals = vec![0; len];
            for i in 0..len {
                let prev_idx = (i as isize - dir + len as isize) % len as isize;
                let (pr, pc) = belt.cells[prev_idx as usize];
                new_vals[i] = self.grid[pr * 20 + pc];
            }
            for i in 0..len {
                let (r, c) = belt.cells[i];
                let val = new_vals[i];
                self.grid[r * 20 + c] = val;
                if val != 999 {
                    self.box_pos[val as usize] = (r as u8, c as u8);
                }
            }

            while self.next_to_remove < 400 && self.grid[10] == self.next_to_remove as u16 {
                self.grid[10] = 999;
                self.next_to_remove += 1;
            }
        }
    }
}

fn get_next_pos(belt: &Belt, r: usize, c: usize, d: isize) -> (usize, usize) {
    let len = belt.cells.len();
    let mut idx = 0;
    for i in 0..len {
        if belt.cells[i] == (r, c) { idx = i; break; }
    }
    let next_idx = (idx as isize + d + len as isize) % len as isize;
    belt.cells[next_idx as usize]
}

fn precompute_dist(n: usize, exit_r: usize, exit_c: usize, belts: &[Belt]) -> Vec<Vec<usize>> {
    let mut dist = vec![vec![999; n]; n];
    let mut queue = VecDeque::new();
    queue.push_back((exit_r, exit_c));
    dist[exit_r][exit_c] = 0;

    while let Some((r, c)) = queue.pop_front() {
        let h_belt_id = r / 2;
        let v_belt_id = 10 + c / 2;
        for &(b_id, d) in &[(h_belt_id, 1), (h_belt_id, -1), (v_belt_id, 1), (v_belt_id, -1)] {
            let (nr, nc) = get_next_pos(&belts[b_id], r, c, d);
            if dist[nr][nc] == 999 {
                dist[nr][nc] = dist[r][c] + 1;
                queue.push_back((nr, nc));
            }
        }
    }
    dist
}

fn eval(state: &State, initial_target: usize, dist: &Vec<Vec<usize>>, ops: usize) -> i32 {
    let mut score = (ops as i32) * 100;
    
    if state.next_to_remove > initial_target {
        let removed_count = state.next_to_remove - initial_target;
        score -= (removed_count as i32) * 100000;
    }

    if state.next_to_remove < 400 {
        let (r, c) = state.box_pos[state.next_to_remove];
        score += dist[r as usize][c as usize] as i32 * 100;
    }
    if state.next_to_remove + 1 < 400 {
        let (r, c) = state.box_pos[state.next_to_remove + 1];
        score += dist[r as usize][c as usize] as i32 * 50;
    }
    if state.next_to_remove + 2 < 400 {
        let (r, c) = state.box_pos[state.next_to_remove + 2];
        score += dist[r as usize][c as usize] as i32 * 20;
    }
    if state.next_to_remove + 3 < 400 {
        let (r, c) = state.box_pos[state.next_to_remove + 3];
        score += dist[r as usize][c as usize] as i32 * 10;
    }
    
    score
}

fn run_beam_search(start_state: &State, max_depth: usize, beam_width: usize, belts: &[Belt], dist: &Vec<Vec<usize>>, initial_target: usize, start_ops: usize) -> i32 {
    let mut beam = vec![(start_state.clone(), start_ops)];
    let mut best_overall_score = eval(start_state, initial_target, dist, start_ops);

    for _ in 0..max_depth {
        let mut next_beam = vec![];
        for (state, ops) in beam {
            let target = state.next_to_remove;
            if target >= 400 { continue; }
            
            let (r, c) = state.box_pos[target];
            let mut cands = vec![(r as usize / 2, 1isize), (r as usize / 2, -1), (10 + c as usize / 2, 1), (10 + c as usize / 2, -1)];
            
            if target + 1 < 400 {
                let (r2, c2) = state.box_pos[target + 1];
                for &op in &[(r2 as usize / 2, 1isize), (r2 as usize / 2, -1isize), (10 + c2 as usize / 2, 1isize), (10 + c2 as usize / 2, -1isize)] {
                    if !cands.contains(&op) { cands.push(op); }
                }
            }

            for &(b, d) in &cands {
                let mut new_state = state.clone();
                new_state.apply_shift(b, d, belts);
                let new_score = eval(&new_state, initial_target, dist, ops + 1);
                if new_score < best_overall_score {
                    best_overall_score = new_score;
                }
                next_beam.push((new_state, ops + 1, new_score));
            }
        }
        
        if next_beam.is_empty() { break; }
        
        next_beam.sort_unstable_by_key(|&(_, _, score)| score);
        next_beam.truncate(beam_width);
        
        beam = next_beam.into_iter().map(|(s, ops, _)| (s, ops)).collect();
    }
    
    best_overall_score
}

fn main() {
    let start_time = Instant::now();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n_line = lines.next();
    if n_line.is_none() { return; }
    let n: usize = n_line.unwrap().unwrap().trim().parse().unwrap();
    let mut initial_grid = vec![999u16; n * n];
    let mut initial_box_pos = vec![(0, 0); n * n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<u16> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        for j in 0..n {
            initial_grid[i * n + j] = row[j];
            initial_box_pos[row[j] as usize] = (i as u8, j as u8);
        }
    }

    let exit_r = 0;
    let exit_c = n / 2;

    // Define belts
    let mut belts = vec![];
    for k in 0..(n / 2) {
        let mut cells = vec![];
        let r1 = 2 * k;
        let r2 = 2 * k + 1;
        for j in 0..n { cells.push((r1, j)); }
        for j in (0..n).rev() { cells.push((r2, j)); }
        belts.push(Belt { cells });
    }
    for k in 0..(n / 2) {
        let mut cells = vec![];
        let c1 = 2 * k;
        let c2 = 2 * k + 1;
        for i in 0..n { cells.push((i, c1)); }
        for i in (0..n).rev() { cells.push((i, c2)); }
        belts.push(Belt { cells });
    }

    let dist = precompute_dist(n, exit_r, exit_c, &belts);

    let mut current_state = State {
        grid: initial_grid,
        box_pos: initial_box_pos,
        next_to_remove: 0,
    };

    // Initial removal
    while current_state.next_to_remove < n * n && current_state.grid[exit_c] == current_state.next_to_remove as u16 {
        current_state.grid[exit_c] = 999;
        current_state.next_to_remove += 1;
    }

    let mut operations = vec![];

    while current_state.next_to_remove < n * n {
        let elapsed = start_time.elapsed().as_secs_f64();
        let progress = current_state.next_to_remove as f64 / 400.0;
        
        let mut beam_width = 30;
        let mut max_depth = 6;
        
        if elapsed > 1.85 {
            beam_width = 1;
            max_depth = 1;
        } else if elapsed / 1.8 > progress {
            beam_width = 10;
            max_depth = 3;
        }

        let target = current_state.next_to_remove;
        let (r, c) = current_state.box_pos[target];
        let mut cands = vec![(r as usize / 2, 1isize), (r as usize / 2, -1), (10 + c as usize / 2, 1), (10 + c as usize / 2, -1)];
        
        if target + 1 < 400 {
            let (r2, c2) = current_state.box_pos[target + 1];
            for &op in &[(r2 as usize / 2, 1isize), (r2 as usize / 2, -1isize), (10 + c2 as usize / 2, 1isize), (10 + c2 as usize / 2, -1isize)] {
                if !cands.contains(&op) { cands.push(op); }
            }
        }
        if target + 2 < 400 {
            let (r2, c2) = current_state.box_pos[target + 2];
            for &op in &[(r2 as usize / 2, 1isize), (r2 as usize / 2, -1isize), (10 + c2 as usize / 2, 1isize), (10 + c2 as usize / 2, -1isize)] {
                if !cands.contains(&op) { cands.push(op); }
            }
        }

        let mut best_first_op = (0, 0);
        let mut best_score = i32::MAX;

        for &first_op in &cands {
            let mut s = current_state.clone();
            s.apply_shift(first_op.0, first_op.1, &belts);
            let score = run_beam_search(&s, max_depth, beam_width, &belts, &dist, target, 1);
            if score < best_score {
                best_score = score;
                best_first_op = first_op;
            }
        }

        // Apply best move
        current_state.apply_shift(best_first_op.0, best_first_op.1, &belts);
        operations.push(best_first_op);
    }

    // Output
    println!("{}", belts.len());
    for belt in &belts {
        print!("{}", belt.cells.len());
        for &(r, c) in &belt.cells {
            print!(" {} {}", r, c);
        }
        println!();
    }
    println!("{}", operations.len());
    for &(id, d) in &operations {
        println!("{} {}", id, d);
    }
}
