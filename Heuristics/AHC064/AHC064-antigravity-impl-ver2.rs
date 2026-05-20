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
        let mut best_score = i64::MIN;
        
        for i in 0..10 {
            if i == target_r { continue; }
            let space = 15 - state.d[i].len();
            if space == 0 { continue; }
            
            let dist = i.abs_diff(j);
            let score = if space >= remaining {
                1000 - dist as i64
            } else {
                space as i64 * 10 - dist as i64
            };
            
            if score > best_score {
                best_score = score;
                best_i = i;
            }
        }
        
        let space = 15 - state.d[best_i].len();
        let take = std::cmp::min(remaining, space);
        issue_move(state, moves, 1, best_i, j, take);
        remaining -= take;
    }
}

fn move_top_d_from_d(state: &mut State, moves: &mut Vec<Move>, i: usize, d: usize) {
    let mut remaining = d;
    while remaining > 0 {
        let mut best_j = 0;
        let mut best_score = i64::MIN;
        
        for j in 0..10 {
            let space = 20 - state.s[j].len();
            if space == 0 { continue; }
            
            let dist = i.abs_diff(j);
            let score = if space >= remaining {
                1000 - dist as i64
            } else {
                space as i64 * 10 - dist as i64
            };
            
            if score > best_score {
                best_score = score;
                best_j = j;
            }
        }
        
        let space = 20 - state.s[best_j].len();
        let take = std::cmp::min(remaining, space);
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
            let (track_type, j, d) = find_car(&state, w).unwrap();
            
            let mut cost = d * 100;
            if track_type == 0 { cost += 50; }
            
            if d > 0 {
                let mut min_dist = usize::MAX;
                if track_type == 1 { // S track -> needs D buf
                    for i in 0..10 {
                        if i == r { continue; }
                        if 15 - state.d[i].len() >= d {
                            min_dist = std::cmp::min(min_dist, i.abs_diff(j));
                        }
                    }
                } else { // D track -> needs S buf
                    for s_j in 0..10 {
                        if 20 - state.s[s_j].len() >= d + 1 {
                            min_dist = std::cmp::min(min_dist, s_j.abs_diff(j));
                        }
                    }
                }
                if min_dist != usize::MAX {
                    cost += min_dist;
                } else {
                    cost += 20; // Needs split
                }
            }
            
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

        if track_type == 1 { // S track
            if d > 0 {
                move_top_d_from_s(&mut state, &mut sequential_moves, t_idx, d, r);
            }
            let (new_track_type, new_t_idx, new_d) = find_car(&state, w).unwrap();
            assert_eq!(new_track_type, 1);
            assert_eq!(new_d, 0);

            let mut k = 1;
            while k < state.s[new_t_idx].len() && 
                  finalized[r] + k < 10 && 
                  state.s[new_t_idx][k] == 10 * r + finalized[r] + k {
                k += 1;
            }
            
            issue_move(&mut state, &mut sequential_moves, 1, r, new_t_idx, k);
            finalized[r] += k;

        } else { // D track
            if d > 0 {
                // Magic extraction: move d+1 to S
                move_top_d_from_d(&mut state, &mut sequential_moves, t_idx, d + 1);
            } else {
                move_top_d_from_d(&mut state, &mut sequential_moves, t_idx, 1);
            }
            
            let (new_track_type, new_t_idx, new_d) = find_car(&state, w).unwrap();
            assert_eq!(new_track_type, 1);
            assert_eq!(new_d, 0);

            let mut k = 1;
            while k < state.s[new_t_idx].len() && 
                  finalized[r] + k < 10 && 
                  state.s[new_t_idx][k] == 10 * r + finalized[r] + k {
                k += 1;
            }

            issue_move(&mut state, &mut sequential_moves, 1, r, new_t_idx, k);
            finalized[r] += k;
        }
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
