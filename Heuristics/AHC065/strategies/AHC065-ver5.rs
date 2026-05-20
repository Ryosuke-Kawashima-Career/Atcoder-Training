use std::collections::VecDeque;
use std::io::{self, BufRead};

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
        let belt = &belts[belt_id];
        let len = belt.cells.len();

        let mut new_vals = [0u16; 80];
        if d > 0 {
            for i in 0..len {
                let prev_idx = if i == 0 { len - 1 } else { i - 1 };
                let (pr, pc) = belt.cells[prev_idx];
                new_vals[i] = self.grid[pr * 20 + pc];
            }
        } else {
            for i in 0..len {
                let prev_idx = if i == len - 1 { 0 } else { i + 1 };
                let (pr, pc) = belt.cells[prev_idx];
                new_vals[i] = self.grid[pr * 20 + pc];
            }
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

fn get_next_pos(belt: &Belt, r: usize, c: usize, d: isize) -> (usize, usize) {
    let len = belt.cells.len();
    let mut idx = 0;
    for i in 0..len {
        if belt.cells[i] == (r, c) { idx = i; break; }
    }
    let next_idx = (idx as isize + d + len as isize) % len as isize;
    belt.cells[next_idx as usize]
}

fn precompute_dist(exit_r: usize, exit_c: usize, belts: &[Belt]) -> Vec<usize> {
    let mut dist = vec![999; 400];
    let mut queue = VecDeque::new();
    queue.push_back((exit_r, exit_c));
    dist[exit_r * 20 + exit_c] = 0;

    while let Some((r, c)) = queue.pop_front() {
        let h_belt_id = r / 2;
        let v_belt_id = 10 + c / 2;
        for &(b_id, d) in &[(h_belt_id, 1), (h_belt_id, -1), (v_belt_id, 1), (v_belt_id, -1)] {
            let (nr, nc) = get_next_pos(&belts[b_id], r, c, d);
            if dist[nr * 20 + nc] == 999 {
                dist[nr * 20 + nc] = dist[r * 20 + c] + 1;
                queue.push_back((nr, nc));
            }
        }
    }
    dist
}

fn eval(state: &State, initial_target: usize, dist: &[usize], ops: usize) -> i32 {
    let mut score = (ops as i32) * 100;
    
    if state.next_to_remove > initial_target {
        let removed_count = state.next_to_remove - initial_target;
        score -= (removed_count as i32) * 1000000;
    }

    let weights = [100, 30, 15, 10, 5, 2, 1, 1, 1, 1, 1, 1];
    
    for i in 0..12 {
        let box_id = state.next_to_remove + i;
        if box_id < 400 {
            let (r, c) = state.box_pos[box_id];
            score += dist[r as usize * 20 + c as usize] as i32 * weights[i];
        }
    }
    
    score
}

#[derive(Clone)]
struct BeamNode {
    state: State,
    first_op: (usize, isize),
    last_op: (usize, isize),
    score: i32,
}

fn get_best_move(
    start_state: &State, 
    belts: &[Belt], 
    dist: &[usize], 
    beam_width: usize, 
    budget: usize,
    expansions_used: &mut usize
) -> (usize, isize) {
    let target = start_state.next_to_remove;
    if target >= 400 { return (0, 0); }

    let mut active_belts = Vec::new();
    for i in 0..6 {
        if target + i < 400 {
            let (r, c) = start_state.box_pos[target + i];
            let h = (r / 2) as usize;
            let v = 10 + (c / 2) as usize;
            if !active_belts.contains(&h) { active_belts.push(h); }
            if !active_belts.contains(&v) { active_belts.push(v); }
        }
    }

    let mut beam = Vec::new();
    let mut best_score = i32::MAX;
    let mut best_first_op = (0, 0);

    let mut local_expansions = 0;

    // Depth 1
    for &b in &active_belts {
        for &d in &[1isize, -1isize] {
            let mut new_state = start_state.clone();
            new_state.apply_shift(b, d, belts);
            local_expansions += 1;
            let score = eval(&new_state, target, dist, 1);
            if score < best_score {
                best_score = score;
                best_first_op = (b, d);
            }
            beam.push(BeamNode {
                state: new_state,
                first_op: (b, d),
                last_op: (b, d),
                score,
            });
        }
    }

    beam.sort_unstable_by_key(|n| n.score);
    beam.truncate(beam_width);

    let mut ops = 2;
    // Anytime loop based on expansion budget
    while local_expansions < budget {
        let mut next_beam = Vec::with_capacity(beam.len() * active_belts.len() * 2);
        
        for node in &beam {
            if local_expansions >= budget { break; }

            let mut node_active = Vec::new();
            for i in 0..6 {
                if node.state.next_to_remove + i < 400 {
                    let (r, c) = node.state.box_pos[node.state.next_to_remove + i];
                    let h = (r / 2) as usize;
                    let v = 10 + (c / 2) as usize;
                    if !node_active.contains(&h) { node_active.push(h); }
                    if !node_active.contains(&v) { node_active.push(v); }
                }
            }
            
            for &b in &node_active {
                if local_expansions >= budget { break; }
                for &d in &[1isize, -1isize] {
                    // Prune immediate reversal
                    if b == node.last_op.0 && d == -node.last_op.1 {
                        continue;
                    }

                    let mut new_state = node.state.clone();
                    new_state.apply_shift(b, d, belts);
                    local_expansions += 1;
                    
                    let score = eval(&new_state, target, dist, ops);
                    
                    if score < best_score {
                        best_score = score;
                        best_first_op = node.first_op;
                    }
                    
                    next_beam.push(BeamNode {
                        state: new_state,
                        first_op: node.first_op,
                        last_op: (b, d),
                        score,
                    });
                }
            }
        }
        
        if next_beam.is_empty() { break; }
        
        next_beam.sort_unstable_by_key(|n| n.score);
        next_beam.truncate(beam_width);
        beam = next_beam;
        ops += 1;
        
        // Prevent infinite loops if budget is extremely large
        if ops > 20 { break; }
    }
    
    *expansions_used += local_expansions;
    best_first_op
}

fn main() {
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

    let dist = precompute_dist(exit_r, exit_c, &belts);

    let mut current_state = State {
        grid: initial_grid,
        box_pos: initial_box_pos,
        next_to_remove: 0,
    };

    while current_state.next_to_remove < n * n && current_state.grid[10] == current_state.next_to_remove as u16 {
        current_state.grid[10] = 999;
        current_state.next_to_remove += 1;
    }

    let mut operations = vec![];
    
    let mut total_expansions_used = 0;
    let total_budget: usize = 40_000_000; // 40M evaluations easily fit in 2s.

    while current_state.next_to_remove < n * n {
        let remaining_budget = total_budget.saturating_sub(total_expansions_used);
        
        let ops_per_box = if current_state.next_to_remove > 10 {
            (operations.len() as f64) / (current_state.next_to_remove as f64)
        } else {
            15.0
        };
        
        let est_remaining_ops = (ops_per_box * (400 - current_state.next_to_remove) as f64) as usize;
        let mut budget_for_this_op = remaining_budget / est_remaining_ops.max(50);
        
        // Ensure at least a minimal budget
        if budget_for_this_op < 100 {
            budget_for_this_op = 100;
        }
        
        // Increase budget for this op if we have a lot to spare
        if budget_for_this_op > 20000 {
            budget_for_this_op = 20000;
        }

        let best_first_op = get_best_move(
            &current_state, 
            &belts, 
            &dist, 
            50, // width 50
            budget_for_this_op,
            &mut total_expansions_used
        );
        
        current_state.apply_shift(best_first_op.0, best_first_op.1, &belts);
        operations.push(best_first_op);
    }

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
