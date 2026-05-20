use std::collections::hash_map::DefaultHasher;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Move {
    type_: usize,
    i: usize,
    j: usize,
    k: usize,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    d: Vec<Vec<usize>>,
    s: Vec<Vec<usize>>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.d.hash(state);
        self.s.hash(state);
    }
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

#[derive(Clone, Eq, PartialEq)]
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

fn issue_move(
    state: &mut State,
    scheduler: &mut Scheduler,
    moves: &mut Vec<Move>,
    type_: usize,
    i: usize,
    j: usize,
    k: usize,
) {
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
    target_r: usize,
) {
    let mut remaining = d;
    while remaining > 0 {
        let mut best_i = 0;
        let mut best_score = usize::MAX;

        for i in 0..10 {
            if i == target_r {
                continue;
            }
            let space = 15 - state.d[i].len();
            if space == 0 {
                continue;
            }

            let take = std::cmp::min(remaining, space);
            let m = Move {
                type_: 1,
                i,
                j,
                k: take,
            };

            let sim_t = scheduler.simulate(m);
            let penalty = if space >= remaining { 0 } else { 1000000 };
            let dist = i.abs_diff(j);
            let score = penalty + (remaining - take) * 100000 + sim_t * 1000 + dist;

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
    d: usize,
) {
    let mut remaining = d;
    while remaining > 0 {
        let mut best_j = 0;
        let mut best_score = usize::MAX;

        for j in 0..10 {
            let space = 20 - state.s[j].len();
            if space == 0 {
                continue;
            }

            let take = std::cmp::min(remaining, space);
            let m = Move {
                type_: 0,
                i,
                j,
                k: take,
            };

            let sim_t = scheduler.simulate(m);
            let penalty = if space >= remaining { 0 } else { 1000000 };
            let dist = i.abs_diff(j);
            let score = penalty + (remaining - take) * 100000 + sim_t * 1000 + dist;

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
    finalized: &mut Vec<usize>,
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
        while k < state.s[new_t_idx].len()
            && finalized[r] + k < 10
            && state.s[new_t_idx][k] == 10 * r + finalized[r] + k
        {
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
        while k < state.s[new_t_idx].len()
            && finalized[r] + k < 10
            && state.s[new_t_idx][k] == 10 * r + finalized[r] + k
        {
            k += 1;
        }
        issue_move(state, scheduler, moves, 1, r, new_t_idx, k);
        finalized[r] += k;
    }
}

#[derive(Clone, Eq, PartialEq)]
struct SearchNode {
    state: State,
    scheduler: Scheduler,
    moves: Vec<Move>,
    finalized: Vec<usize>,
    score: usize,
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

fn score_node(scheduler: &Scheduler) -> usize {
    scheduler.turns.len() * 10000
        + scheduler.ready_d.iter().sum::<usize>()
        + scheduler.ready_s.iter().sum::<usize>()
}

fn hash_state(state: &State) -> u64 {
    let mut hasher = DefaultHasher::new();
    state.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let start_time = Instant::now();
    let time_limit = Duration::from_millis(1900);

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
        let cars: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        initial_state.d[i] = cars;
    }

    let mut queues: Vec<BinaryHeap<SearchNode>> = vec![BinaryHeap::new(); 101];
    let mut visited: HashSet<u64> = HashSet::new();

    let mut initial_node = SearchNode {
        state: initial_state.clone(),
        scheduler: Scheduler::new(r_count),
        moves: Vec::new(),
        finalized: vec![0; r_count],
        score: 0,
    };
    initial_node.score = score_node(&initial_node.scheduler);
    queues[0].push(initial_node);

    let mut dump_state = initial_state.clone();
    let mut dump_scheduler = Scheduler::new(r_count);
    let mut dump_moves = Vec::new();
    for i in 0..r_count {
        issue_move(
            &mut dump_state,
            &mut dump_scheduler,
            &mut dump_moves,
            0,
            i,
            i,
            10,
        );
    }
    let mut dump_node = SearchNode {
        state: dump_state,
        scheduler: dump_scheduler,
        moves: dump_moves,
        finalized: vec![0; r_count],
        score: 0,
    };
    dump_node.score = score_node(&dump_node.scheduler);
    queues[0].push(dump_node);

    let mut best_final_node = None;
    let mut best_final_score = usize::MAX;

    loop {
        if start_time.elapsed() >= time_limit {
            break;
        }

        let mut expanded_any = false;
        for n in 0..100 {
            let mut popped = None;
            while let Some(node) = queues[n].pop() {
                let h = hash_state(&node.state);
                if visited.insert(h) {
                    popped = Some(node);
                    break;
                }
            }

            if let Some(node) = popped {
                expanded_any = true;
                for r in 0..r_count {
                    if node.finalized[r] == 10 {
                        continue;
                    }

                    let mut next_node = node.clone();
                    process_r(
                        &mut next_node.state,
                        &mut next_node.scheduler,
                        &mut next_node.moves,
                        r,
                        &mut next_node.finalized,
                    );

                    next_node.score = score_node(&next_node.scheduler);

                    let mut new_n = 0;
                    for f in &next_node.finalized {
                        new_n += f;
                    }
                    if new_n > 100 {
                        new_n = 100;
                    }

                    if new_n == 100 {
                        if next_node.score < best_final_score {
                            best_final_score = next_node.score;
                            best_final_node = Some(next_node.clone());
                        }
                    } else {
                        queues[new_n].push(next_node);
                        if queues[new_n].len() > 300 {
                            let mut vec = queues[new_n].drain().collect::<Vec<_>>();
                            vec.sort_unstable_by_key(|n| n.score);
                            vec.truncate(150);
                            queues[new_n] = BinaryHeap::from(vec);
                        }
                    }
                }
            }
        }

        if !expanded_any {
            break;
        }
    }

    if best_final_node.is_none() && !queues[100].is_empty() {
        best_final_node = Some(queues[100].peek().unwrap().clone());
    }

    if let Some(best_node) = best_final_node {
        println!("{}", best_node.scheduler.turns.len());
        for turn in best_node.scheduler.turns {
            println!("{}", turn.len());
            for m in turn {
                println!("{} {} {} {}", m.type_, m.i, m.j, m.k);
            }
        }
    }
}
