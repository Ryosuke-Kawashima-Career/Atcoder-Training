use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
struct Move {
    type_: usize,
    i: usize,
    j: usize,
    k: usize,
}

#[derive(Clone)]
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

#[derive(Clone)]
struct Scheduler {
    turns: Vec<Vec<Move>>,
    ready_d: Vec<usize>,
    ready_s: Vec<usize>,
}

impl Scheduler {
    fn new(r_count: usize) -> Self {
        Self {
            turns: Vec::new(),
            ready_d: vec![0; r_count],
            ready_s: vec![0; r_count],
        }
    }

    fn schedule(&mut self, m: Move) -> usize {
        let mut t = std::cmp::max(self.ready_d[m.i], self.ready_s[m.j]);
        loop {
            if t >= self.turns.len() {
                self.turns.push(Vec::new());
            }

            let mut can_schedule = true;
            for other in &self.turns[t] {
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
                self.turns[t].push(m);
                self.ready_d[m.i] = t + 1;
                self.ready_s[m.j] = t + 1;
                return t;
            } else {
                t += 1;
            }
        }
    }

    fn simulate(&self, moves: &[Move]) -> usize {
        let mut temp_turns = self.turns.clone();
        let mut temp_ready_d = self.ready_d.clone();
        let mut temp_ready_s = self.ready_s.clone();

        let mut last_t = 0;
        for &m in moves {
            let mut t = std::cmp::max(temp_ready_d[m.i], temp_ready_s[m.j]);
            loop {
                if t >= temp_turns.len() {
                    temp_turns.push(Vec::new());
                }

                let mut can_schedule = true;
                for other in &temp_turns[t] {
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
                    temp_turns[t].push(m);
                    temp_ready_d[m.i] = t + 1;
                    temp_ready_s[m.j] = t + 1;
                    last_t = t;
                    break;
                } else {
                    t += 1;
                }
            }
        }
        last_t
    }
}

fn issue_move(state: &mut State, scheduler: &mut Scheduler, moves: &mut Vec<Move>, type_: usize, i: usize, j: usize, k: usize) {
    let m = Move { type_, i, j, k };
    scheduler.schedule(m);
    moves.push(m);
    state.apply_move(type_, i, j, k);
}

fn move_top_d_from_s_with_scheduler(
    state: &mut State,
    scheduler: &mut Scheduler,
    moves: &mut Vec<Move>,
    j: usize,
    d: usize,
    target_r: usize
) {
    let mut remaining = d;
    while remaining > 0 {
        let mut best_i = 0;
        let mut best_score = usize::MAX;

        for i in 0..10 {
            if i == target_r { continue; }
            let space = 15 - state.d[i].len();
            if space == 0 { continue; }

            let take = std::cmp::min(remaining, space);
            let m = Move { type_: 1, i, j, k: take };

            let sim_t = scheduler.simulate(&[m]);
            let penalty = if space >= remaining { 0 } else { 10000 };
            let dist = i.abs_diff(j);

            let score = sim_t * 100000 + penalty + dist;

            if score < best_score {
                best_score = score;
                best_i = i;
            }
        }

        let space = 15 - state.d[best_i].len();
        let take = std::cmp::min(remaining, space);
        issue_move(state, scheduler, moves, 1, best_i, j, take);
        remaining -= take;
    }
}

fn move_top_d_from_d_with_scheduler(
    state: &mut State,
    scheduler: &mut Scheduler,
    moves: &mut Vec<Move>,
    i: usize,
    d: usize
) {
    let mut remaining = d;
    while remaining > 0 {
        let mut best_j = 0;
        let mut best_score = usize::MAX;

        for j in 0..10 {
            let space = 20 - state.s[j].len();
            if space == 0 { continue; }

            let take = std::cmp::min(remaining, space);
            let m = Move { type_: 0, i, j, k: take };

            let sim_t = scheduler.simulate(&[m]);
            let penalty = if space >= remaining { 0 } else { 10000 };
            let dist = i.abs_diff(j);

            let score = sim_t * 100000 + penalty + dist;

            if score < best_score {
                best_score = score;
                best_j = j;
            }
        }

        let space = 20 - state.s[best_j].len();
        let take = std::cmp::min(remaining, space);
        issue_move(state, scheduler, moves, 0, i, best_j, take);
        remaining -= take;
    }
}

fn process_r(
    state: &mut State,
    scheduler: &mut Scheduler,
    moves: &mut Vec<Move>,
    r: usize,
    finalized: &mut Vec<usize>
) {
    let unfinalized = state.d[r].len() - finalized[r];
    if unfinalized > 0 {
        move_top_d_from_d_with_scheduler(state, scheduler, moves, r, unfinalized);
    }

    let w = 10 * r + finalized[r];
    let (track_type, t_idx, d) = find_car(state, w).unwrap();

    if track_type == 1 { // S track
        if d > 0 {
            move_top_d_from_s_with_scheduler(state, scheduler, moves, t_idx, d, r);
        }
        let (new_track_type, new_t_idx, new_d) = find_car(state, w).unwrap();
        assert_eq!(new_track_type, 1);
        assert_eq!(new_d, 0);

        let mut k = 1;
        while k < state.s[new_t_idx].len() && 
              finalized[r] + k < 10 && 
              state.s[new_t_idx][k] == 10 * r + finalized[r] + k {
            k += 1;
        }

        issue_move(state, scheduler, moves, 1, r, new_t_idx, k);
        finalized[r] += k;

    } else { // D track
        if d > 0 {
            move_top_d_from_d_with_scheduler(state, scheduler, moves, t_idx, d + 1);
        } else {
            move_top_d_from_d_with_scheduler(state, scheduler, moves, t_idx, 1);
        }

        let (new_track_type, new_t_idx, new_d) = find_car(state, w).unwrap();
        assert_eq!(new_track_type, 1);
        assert_eq!(new_d, 0);

        let mut k = 1;
        while k < state.s[new_t_idx].len() && 
              finalized[r] + k < 10 && 
              state.s[new_t_idx][k] == 10 * r + finalized[r] + k {
            k += 1;
        }

        issue_move(state, scheduler, moves, 1, r, new_t_idx, k);
        finalized[r] += k;
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

    let mut scheduler = Scheduler::new(r_count);
    let mut sequential_moves = Vec::new();

    // Turn 1: Dump all to S
    for i in 0..r_count {
        issue_move(&mut state, &mut scheduler, &mut sequential_moves, 0, i, i, 10);
    }

    let mut finalized = vec![0; r_count];

    loop {
        let mut best_r = None;
        let mut best_score = usize::MAX;
        let mut best_state = None;
        let mut best_scheduler = None;
        let mut best_moves = None;
        let mut best_finalized = None;

        let mut finished = true;
        for r in 0..r_count {
            if finalized[r] == 10 { continue; }
            finished = false;

            let mut temp_state = state.clone();
            let mut temp_scheduler = scheduler.clone();
            let mut temp_moves = Vec::new();
            let mut temp_finalized = finalized.clone();

            process_r(&mut temp_state, &mut temp_scheduler, &mut temp_moves, r, &mut temp_finalized);

            let makespan = temp_scheduler.turns.len();
            let total_ready: usize = temp_scheduler.ready_d.iter().sum::<usize>() + temp_scheduler.ready_s.iter().sum::<usize>();
            let score = makespan * 10000 + total_ready;

            if score < best_score {
                best_score = score;
                best_r = Some(r);
                best_state = Some(temp_state);
                best_scheduler = Some(temp_scheduler);
                best_moves = Some(temp_moves);
                best_finalized = Some(temp_finalized);
            }
        }

        if finished { break; }

        state = best_state.unwrap();
        scheduler = best_scheduler.unwrap();
        sequential_moves.extend(best_moves.unwrap());
        finalized = best_finalized.unwrap();
    }

    println!("{}", scheduler.turns.len());
    for turn in scheduler.turns {
        println!("{}", turn.len());
        for m in turn {
            println!("{} {} {} {}", m.type_, m.i, m.j, m.k);
        }
    }
}
