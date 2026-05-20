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
            let d_len = self.d[i].len();
            let mut block = self.d[i].split_off(d_len - k);
            block.append(&mut self.s[j]);
            self.s[j] = block;
        } else {
            let mut block = self.s[j].drain(0..k).collect::<Vec<_>>();
            self.d[i].append(&mut block);
        }
    }
}

fn find_car(state: &State, c: usize) -> Option<(usize, usize, usize)> {
    for i in 0..state.d.len() {
        for (depth, &car) in state.d[i].iter().rev().enumerate() {
            if car == c { return Some((0, i, depth)); }
        }
    }
    for j in 0..state.s.len() {
        for (depth, &car) in state.s[j].iter().enumerate() {
            if car == c { return Some((1, j, depth)); }
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

    // O(1) allocation-free simulation
    fn simulate(&self, m: Move) -> usize {
        let mut t = std::cmp::max(self.ready_d[m.i], self.ready_s[m.j]);
        loop {
            if t >= self.turns.len() {
                return t;
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
                return t;
            } else {
                t += 1;
            }
        }
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

            let sim_t = scheduler.simulate(m);
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

            let sim_t = scheduler.simulate(m);
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

    if track_type == 1 {
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

    } else {
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

#[derive(Clone)]
struct SearchNode {
    state: State,
    scheduler: Scheduler,
    moves: Vec<Move>,
    finalized: Vec<usize>,
}

fn score_node(node: &SearchNode) -> usize {
    node.scheduler.turns.len() * 10000 + 
    node.scheduler.ready_d.iter().sum::<usize>() + 
    node.scheduler.ready_s.iter().sum::<usize>()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let r_line = lines.next().unwrap().unwrap();
    let r_count: usize = r_line.trim().parse().unwrap();

    let mut initial_state = State {
        d: vec![vec![]; r_count],
        s: vec![vec![]; r_count],
    };

    for i in 0..r_count {
        let line = lines.next().unwrap().unwrap();
        let cars: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        initial_state.d[i] = cars;
    }

    let beam_width = 300;
    let mut beam: Vec<Vec<SearchNode>> = vec![Vec::new(); 101];

    // Node 1: Lazy approach (No initial dump)
    beam[0].push(SearchNode {
        state: initial_state.clone(),
        scheduler: Scheduler::new(r_count),
        moves: Vec::new(),
        finalized: vec![0; r_count],
    });

    // Node 2: Eager approach (Turn 1 dump)
    let mut dump_state = initial_state.clone();
    let mut dump_scheduler = Scheduler::new(r_count);
    let mut dump_moves = Vec::new();
    for i in 0..r_count {
        issue_move(&mut dump_state, &mut dump_scheduler, &mut dump_moves, 0, i, i, 10);
    }
    beam[0].push(SearchNode {
        state: dump_state,
        scheduler: dump_scheduler,
        moves: dump_moves,
        finalized: vec![0; r_count],
    });

    for n in 0..100 {
        if beam[n].is_empty() { continue; }

        beam[n].sort_by_key(|node| score_node(node));
        if beam[n].len() > beam_width {
            beam[n].truncate(beam_width);
        }

        let current_nodes = beam[n].clone();

        for node in current_nodes {
            for r in 0..r_count {
                if node.finalized[r] == 10 { continue; }

                let mut next_node = node.clone();
                process_r(&mut next_node.state, &mut next_node.scheduler, &mut next_node.moves, r, &mut next_node.finalized);

                let mut new_n = 0;
                for f in &next_node.finalized {
                    new_n += f;
                }
                if new_n > 100 { new_n = 100; }
                beam[new_n].push(next_node);
            }
        }
    }

    beam[100].sort_by_key(|node| node.scheduler.turns.len());
    let best_node = beam[100].remove(0);

    println!("{}", best_node.scheduler.turns.len());
    for turn in best_node.scheduler.turns {
        println!("{}", turn.len());
        for m in turn {
            println!("{} {} {} {}", m.type_, m.i, m.j, m.k);
        }
    }
}
