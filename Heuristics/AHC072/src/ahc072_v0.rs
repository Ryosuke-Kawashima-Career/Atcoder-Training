use proconio::{input, marker::Chars};
use std::collections::{HashSet, VecDeque};

const MAX_HEIGHT: usize = 8;
const MAX_JUMPS: usize = 100_000;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (0, 1), (1, 0), (0, N1)];
const DIRECTIONS: [char; 4] = ['U', 'R', 'D', 'L'];

struct State {
    h: usize,
    w: usize,
    n_colors: usize,
    graph: Vec<Vec<char>>,
    slimes: Vec<Vec<(usize, usize)>>,
    floors: Vec<HashSet<(usize, usize)>>,
    heights: Vec<Vec<usize>>,
}

impl State {
    fn new(n: usize, k: usize, c: &[Vec<char>]) -> Self {
        let mut slimes = vec![vec![]; k];
        let mut floors = vec![HashSet::new(); k];
        let mut heights = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                match c[i][j] {
                    '#' | '.' => {}
                    'a'..='z' => {
                        let color = c[i][j] as usize - 'a' as usize;
                        slimes[color].push((i, j));
                        heights[i][j] = 1;
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
        }
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
        // Move each slime home independently, leaving other slimes in place.
        for color in 0..self.state.n_colors {
            let target_pos = *self.state.floors[color].iter().next().unwrap();
            let slimes = self.state.slimes[color].clone();
            for current_pos in slimes {
                let path = self.bfs(&current_pos, &target_pos);
                if self.jumps.len() + path.len() > MAX_JUMPS {
                    return;
                }
                for mut jump in path {
                    // Only the top slime moves, including when crossing a tower.
                    jump.k_remains = self.state.heights[jump.i][jump.j] - 1;
                    let d = DIRECTIONS.iter().position(|&d| d == jump.d).unwrap();
                    let ni = jump.i.wrapping_add(D4[d].0);
                    let nj = jump.j.wrapping_add(D4[d].1);
                    assert!(self.state.heights[ni][nj] < MAX_HEIGHT);
                    self.state.heights[jump.i][jump.j] -= 1;
                    self.state.heights[ni][nj] += 1;
                    self.jumps.push(jump);
                }
                // BFS first reaches this slime's nest at the end of the path.
                self.state.heights[target_pos.0][target_pos.1] -= 1;
            }
            self.state.slimes[color].clear();
        }
    }

    fn bfs(&self, current_pos: &(usize, usize), target_pos: &(usize, usize)) -> Vec<Jump> {
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; self.state.w]; self.state.h];
        let mut prev_pos = vec![vec![None; self.state.w]; self.state.h];
        queue.push_back(*current_pos);
        visited[current_pos.0][current_pos.1] = true;
        while let Some(pos) = queue.pop_front() {
            if pos == *target_pos {
                break;
            }
            for (di, dj) in D4 {
                let ni = pos.0.wrapping_add(di);
                let nj = pos.1.wrapping_add(dj);
                if ni < self.state.h
                    && nj < self.state.w
                    && self.state.graph[ni][nj] != '#'
                    && !visited[ni][nj]
                {
                    visited[ni][nj] = true;
                    prev_pos[ni][nj] = Some(pos);
                    queue.push_back((ni, nj));
                }
            }
        }
        self.recon_path(&prev_pos, current_pos, target_pos)
    }

    fn recon_path(
        &self,
        prev_pos: &[Vec<Option<(usize, usize)>>],
        start_pos: &(usize, usize),
        target_pos: &(usize, usize),
    ) -> Vec<Jump> {
        let mut path = Vec::new();
        let mut pos = *target_pos;
        while pos != *start_pos {
            let prev = prev_pos[pos.0][pos.1].expect("The nest must be reachable");
            let d = D4
                .iter()
                .position(|&(di, dj)| (prev.0.wrapping_add(di), prev.1.wrapping_add(dj)) == pos)
                .unwrap();
            path.push(Jump {
                i: prev.0,
                j: prev.1,
                k_remains: 0, // Set from the actual tower height in solve().
                d: DIRECTIONS[d],
                l_block: 1,
            });
            pos = prev;
        }
        path.reverse();
        path
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
    let state = State::new(n, k, &c);
    let mut solver = Solver::new(state);
    solver.solve();
    solver.output();
}
