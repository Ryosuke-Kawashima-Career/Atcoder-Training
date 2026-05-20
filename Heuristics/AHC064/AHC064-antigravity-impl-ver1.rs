use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
struct Move {
    type_: usize,
    i: usize,
    j: usize,
    k: usize,
}

struct State {
    d: Vec<Vec<usize>>,
    s: Vec<Vec<usize>>,
}

impl State {
    fn apply_move(&mut self, type_: usize, i: usize, j: usize, k: usize) {
        if type_ == 0 {
            // D_i -> S_j
            let d_len = self.d[i].len();
            let mut block = self.d[i].split_off(d_len - k);
            block.append(&mut self.s[j]);
            self.s[j] = block;
        } else {
            // S_j -> D_i
            let mut block = self.s[j].drain(0..k).collect::<Vec<_>>();
            self.d[i].append(&mut block);
        }
    }
}

fn find_car(state: &State, c: usize) -> Option<(usize, usize, usize)> {
    for i in 0..state.d.len() {
        for (depth, &car) in state.d[i].iter().rev().enumerate() {
            if car == c {
                return Some((0, i, depth));
            }
        }
    }
    for j in 0..state.s.len() {
        for (depth, &car) in state.s[j].iter().enumerate() {
            if car == c {
                return Some((1, j, depth));
            }
        }
    }
    None
}

fn issue_move(state: &mut State, moves: &mut Vec<Move>, type_: usize, i: usize, j: usize, k: usize) {
    moves.push(Move { type_, i, j, k });
    state.apply_move(type_, i, j, k);
}

fn move_top_d_from_s(state: &mut State, moves: &mut Vec<Move>, j: usize, d: usize, target_r: usize) {
    let mut remaining = d;
    while remaining > 0 {
        let mut best_i = 0;
        let mut best_space = 0;
        for i in 0..10 {
            if i == target_r { continue; }
            let space = 15 - state.d[i].len();
            if space > best_space {
                best_space = space;
                best_i = i;
            }
        }
        let take = std::cmp::min(remaining, best_space);
        issue_move(state, moves, 1, best_i, j, take);
        remaining -= take;
    }
}

fn move_top_d_from_d(state: &mut State, moves: &mut Vec<Move>, i: usize, d: usize) {
    let mut remaining = d;
    while remaining > 0 {
        let mut best_j = 0;
        let mut best_space = 0;
        for j in 0..10 {
            let space = 20 - state.s[j].len();
            if space > best_space {
                best_space = space;
                best_j = j;
            }
        }
        let take = std::cmp::min(remaining, best_space);
        issue_move(state, moves, 0, i, best_j, take);
        remaining -= take;
    }
}

fn move_unfinalized_out_of_d(state: &mut State, moves: &mut Vec<Move>, r: usize, finalized_count: usize) {
    let unfinalized = state.d[r].len() - finalized_count;
    if unfinalized > 0 {
        move_top_d_from_d(state, moves, r, unfinalized);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let r_line = lines.next().unwrap().unwrap();
    let r_count: usize = r_line.trim().parse().unwrap();

    let mut state = State {
        d: vec![vec![]; r_count],
        s: vec![vec![]; r_count],
    };

    for i in 0..r_count {
        let line = lines.next().unwrap().unwrap();
        let cars: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        state.d[i] = cars;
    }

    let mut sequential_moves = Vec::new();

    // Turn 1: Dump all to S
    for i in 0..r_count {
        issue_move(&mut state, &mut sequential_moves, 0, i, i, 10);
    }

    let mut finalized = vec![0; r_count];

    loop {
        let mut best_r = None;
        let mut best_cost = usize::MAX;

        let mut finished = true;
        for r in 0..r_count {
            if finalized[r] == 10 { continue; }
            finished = false;
            let w = 10 * r + finalized[r];
            let (track_type, _, d) = find_car(&state, w).unwrap();
            
            let cost = d * 2 + if track_type == 0 { 1 } else { 0 };
            if cost < best_cost {
                best_cost = cost;
                best_r = Some(r);
            }
        }

        if finished { break; }

        let r = best_r.unwrap();
        
        // Ensure D_r is clean
        let unfinalized = state.d[r].len() - finalized[r];
        if unfinalized > 0 {
            move_unfinalized_out_of_d(&mut state, &mut sequential_moves, r, finalized[r]);
            continue; // Re-evaluate costs since state changed
        }

        let w = 10 * r + finalized[r];
        let (track_type, t_idx, d) = find_car(&state, w).unwrap();

        if d > 0 {
            if track_type == 1 { // S track
                move_top_d_from_s(&mut state, &mut sequential_moves, t_idx, d, r);
            } else { // D track
                move_top_d_from_d(&mut state, &mut sequential_moves, t_idx, d);
            }
        }

        let (track_type, t_idx, d) = find_car(&state, w).unwrap();
        assert_eq!(d, 0);

        if track_type == 1 { // S track
            issue_move(&mut state, &mut sequential_moves, 1, r, t_idx, 1);
        } else { // D track
            let mut best_j = 0;
            let mut best_space = 0;
            for j in 0..10 {
                let space = 20 - state.s[j].len();
                if space > best_space {
                    best_space = space;
                    best_j = j;
                }
            }
            issue_move(&mut state, &mut sequential_moves, 0, t_idx, best_j, 1);
            issue_move(&mut state, &mut sequential_moves, 1, r, best_j, 1);
        }

        finalized[r] += 1;
    }

    // Schedule moves
    let mut turns: Vec<Vec<Move>> = Vec::new();
    let mut ready_d = vec![0; r_count];
    let mut ready_s = vec![0; r_count];

    for m in sequential_moves {
        let mut t = std::cmp::max(ready_d[m.i], ready_s[m.j]);
        loop {
            if t >= turns.len() {
                turns.push(Vec::new());
            }
            
            let mut can_schedule = true;
            for other in &turns[t] {
                if other.i == m.i || other.j == m.j {
                    can_schedule = false;
                    break;
                }
                if (m.i < other.i && m.j >= other.j) || (m.i > other.i && m.j <= other.j) {
                    can_schedule = false;
                    break;
                }
            }
            
            if can_schedule {
                turns[t].push(m);
                ready_d[m.i] = t + 1;
                ready_s[m.j] = t + 1;
                break;
            } else {
                t += 1;
            }
        }
    }

    println!("{}", turns.len());
    for turn in turns {
        println!("{}", turn.len());
        for m in turn {
            println!("{} {} {} {}", m.type_, m.i, m.j, m.k);
        }
    }
}
