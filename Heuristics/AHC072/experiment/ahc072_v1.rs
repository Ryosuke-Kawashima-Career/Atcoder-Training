use proconio::{input, marker::Chars};
use std::collections::{HashSet, VecDeque};

const MAX_HEIGHT: usize = 8;
const MAX_JUMPS: usize = 100_000;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (0, 1), (1, 0), (0, N1)];
const DIRECTIONS: [char; 4] = ['U', 'R', 'D', 'L'];
type Pos = (usize, usize);

#[derive(Debug, Copy, Clone)]
struct Slime {
    id: usize,
    color: usize,
    pos: Pos,
    // One-based position in its tower; zero means it has returned home.
    height: usize,
}

struct State {
    h: usize,
    w: usize,
    n_colors: usize,
    graph: Vec<Vec<char>>,
    slimes: Vec<Slime>,
    floors: Vec<HashSet<Pos>>,
    heights: Vec<Vec<usize>>,
    // Slime IDs, ordered from bottom to top.
    towers: Vec<Vec<Vec<usize>>>,
}

impl State {
    fn new(n: usize, k: usize, c: &[Vec<char>]) -> Self {
        let mut slimes = Vec::new();
        let mut floors = vec![HashSet::new(); k];
        let mut heights = vec![vec![0; n]; n];
        let mut towers = vec![vec![Vec::new(); n]; n];
        for i in 0..n {
            for j in 0..n {
                match c[i][j] {
                    '#' | '.' => {}
                    'a'..='z' => {
                        let color = c[i][j] as usize - 'a' as usize;
                        let id = slimes.len();
                        slimes.push(Slime {
                            id,
                            color,
                            pos: (i, j),
                            height: 1,
                        });
                        heights[i][j] = 1;
                        towers[i][j].push(id);
                    }
                    'A'..='Z' => {
                        let color = c[i][j] as usize - 'A' as usize;
                        floors[color].insert((i, j));
                    }
                    _ => unreachable!(),
                }
            }
        }
        Self {
            h: n,
            w: n,
            n_colors: k,
            graph: c.to_vec(),
            slimes,
            floors,
            heights,
            towers,
        }
    }

    fn top_count(&self, pos: Pos, color: usize) -> usize {
        self.towers[pos.0][pos.1]
            .iter()
            .rev()
            .take_while(|&&id| self.slimes[id].color == color)
            .count()
    }

    fn return_home(&mut self, pos: Pos) {
        let cell = self.graph[pos.0][pos.1];
        if cell.is_ascii_uppercase() {
            let color = cell as usize - 'A' as usize;
            while let Some(&id) = self.towers[pos.0][pos.1].last() {
                if self.slimes[id].color != color {
                    break;
                }
                self.towers[pos.0][pos.1].pop();
                self.slimes[id].height = 0;
            }
        }
        self.heights[pos.0][pos.1] = self.towers[pos.0][pos.1].len();
    }

    fn apply_jump(&mut self, jump: &Jump) {
        let d = DIRECTIONS.iter().position(|&d| d == jump.d).unwrap();
        let source = (jump.i, jump.j);
        let dest = (
            source.0.wrapping_add(D4[d].0),
            source.1.wrapping_add(D4[d].1),
        );
        assert_eq!(jump.l_block, 1);
        assert!(dest.0 < self.h && dest.1 < self.w && self.graph[dest.0][dest.1] != '#');
        assert!(jump.k_remains < self.heights[source.0][source.1]);
        let moving_count = self.heights[source.0][source.1] - jump.k_remains;
        assert!(self.heights[dest.0][dest.1] + moving_count <= MAX_HEIGHT);
        let mut moving = self.towers[source.0][source.1].split_off(jump.k_remains);
        moving.reverse();
        for id in moving {
            self.towers[dest.0][dest.1].push(id);
            let slime = &mut self.slimes[id];
            debug_assert_eq!(slime.id, id);
            slime.pos = dest;
            slime.height = self.towers[dest.0][dest.1].len();
        }
        self.return_home(source);
        self.return_home(dest);
    }
}

#[derive(Clone)]
struct Jump {
    i: usize,
    j: usize,
    k_remains: usize,
    d: char,
    l_block: usize,
}

struct Solver {
    state: State,
    jumps: Vec<Jump>,
}

impl Solver {
    fn new(state: State) -> Self {
        Self {
            state,
            jumps: Vec::new(),
        }
    }

    fn solve(&mut self) {
        for color in 0..self.state.n_colors {
            let target = *self.state.floors[color].iter().next().unwrap();
            let distances = self.bfs(target);
            if !self.same_gradation_path(color, &distances) {
                return;
            }
            debug_assert!(self
                .state
                .slimes
                .iter()
                .all(|s| s.color != color || s.height == 0));
        }
    }

    // One reverse BFS supplies distances for all slimes of this color.
    fn bfs(&self, target: Pos) -> Vec<Vec<usize>> {
        let mut queue = VecDeque::new();
        let mut distances = vec![vec![usize::MAX; self.state.w]; self.state.h];
        distances[target.0][target.1] = 0;
        queue.push_back(target);
        while let Some(pos) = queue.pop_front() {
            for (di, dj) in D4 {
                let ni = pos.0.wrapping_add(di);
                let nj = pos.1.wrapping_add(dj);
                if ni < self.state.h
                    && nj < self.state.w
                    && self.state.graph[ni][nj] != '#'
                    && distances[ni][nj] == usize::MAX
                {
                    distances[ni][nj] = distances[pos.0][pos.1] + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
        distances
    }

    fn same_gradation_path(&mut self, color: usize, distances: &[Vec<usize>]) -> bool {
        let mut cells = Vec::new();
        for (i, row) in distances.iter().enumerate() {
            for (j, &distance) in row.iter().enumerate() {
                if distance != usize::MAX && distance > 0 {
                    cells.push((distance, (i, j)));
                }
            }
        }
        // All arrivals from farther cells are collected before processing a cell.
        cells.sort_unstable_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        for (_, pos) in cells {
            if !self.drain_cell(pos, color, distances) {
                return false;
            }
        }
        true
    }

    // Move the matching top block one level downhill. Other colors stay put.
    // If the next cell is full, recursively move its matching block downhill first.
    fn drain_cell(&mut self, pos: Pos, color: usize, distances: &[Vec<usize>]) -> bool {
        while self.state.top_count(pos, color) > 0 {
            if self.jumps.len() == MAX_JUMPS {
                return false;
            }
            let mut best = None;
            for (d, (di, dj)) in D4.iter().copied().enumerate() {
                let dest = (pos.0.wrapping_add(di), pos.1.wrapping_add(dj));
                if dest.0 >= self.state.h
                    || dest.1 >= self.state.w
                    || distances[dest.0][dest.1] == usize::MAX
                    || distances[dest.0][dest.1] + 1 != distances[pos.0][pos.1]
                {
                    continue;
                }
                let matching = self.state.top_count(dest, color);
                let room = MAX_HEIGHT - self.state.heights[dest.0][dest.1];
                // Prefer an available cell, then convergence with the largest block.
                let key = (room > 0, matching, room);
                if best.as_ref().is_none_or(|&(old_key, _, _)| key > old_key) {
                    best = Some((key, d, dest));
                }
            }
            let (_, d, dest) = best.expect("Every non-nest floor has a downhill neighbor");
            if self.state.heights[dest.0][dest.1] == MAX_HEIGHT {
                // Unprocessed colors have at most one slime per cell, so a full
                // tower necessarily has a movable block of the current color.
                assert!(self.state.top_count(dest, color) > 0);
                if !self.drain_cell(dest, color, distances) {
                    return false;
                }
                continue;
            }
            let count = self
                .state
                .top_count(pos, color)
                .min(MAX_HEIGHT - self.state.heights[dest.0][dest.1]);
            let jump = Jump {
                i: pos.0,
                j: pos.1,
                k_remains: self.state.heights[pos.0][pos.1] - count,
                d: DIRECTIONS[d],
                l_block: 1,
            };
            self.state.apply_jump(&jump);
            self.jumps.push(jump);
        }
        true
    }

    fn output(&self) {
        for jump in &self.jumps {
            println!(
                "{} {} {} {} {}",
                jump.i, jump.j, jump.k_remains, jump.d, jump.l_block
            );
        }
    }
}

fn main() {
    input! {n: usize, k: usize, c: [Chars; n]}
    let mut solver = Solver::new(State::new(n, k, &c));
    solver.solve();
    solver.output();
}
