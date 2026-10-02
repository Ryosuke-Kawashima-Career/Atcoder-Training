use proconio::{input, marker::Chars};
use std::collections::{HashSet, VecDeque};
const MAX_HEIGHT: usize = 8;
const MAX_JUMPS: usize = 100_000;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (0, 1), (1, 0), (0, N1)];
enum Dir {
    U = 0,
    R = 1,
    D = 2,
    L = 3,
}
struct State {
    H: usize,
    W: usize,
    n_colors: usize,
    graph: Vec<Vec<char>>,
    slimes: Vec<Vec<(usize, usize)>>,
    floors: Vec<HashSet<(usize, usize)>>,
}
impl State {
    fn new(n: usize, k: usize, c: &Vec<Vec<char>>) -> Self {
        let H: usize = n;
        let W: usize = n;
        let c: Vec<Vec<char>> = c.to_vec();
        let n_colors: usize = k;
        let mut slimes: Vec<Vec<(usize, usize)>> = vec![vec![]; k];
        let mut floors: Vec<HashSet<(usize, usize)>> = vec![HashSet::new(); k];
        for i in 0..H {
            for j in 0..W {
                match c[i][j] {
                    '#' => {
                        continue;
                    }
                    '.' => {
                        continue;
                    }
                    'a'..='z' => {
                        let color: usize = c[i][j] as usize - 'a' as usize;
                        slimes[color].push((i, j));
                    }
                    'A'..='Z' => {
                        let color: usize = c[i][j] as usize - 'A' as usize;
                        floors[color].insert((i, j));
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
        Self {
            H,
            W,
            n_colors,
            graph: c,
            slimes,
            floors,
        }
    }
}
#[derive(Clone)]
struct Jump {
    i: usize,
    j: usize,
    K_remains: usize,
    d: char,
    l_block: usize,
}
struct Solver {
    state: State,
    jumps: Vec<Jump>,
}
impl Solver {
    fn new(state: State) -> Self {
        let jumps = Vec::new();
        Self { state, jumps }
    }
    fn solve(state: State) {
        // Move each slime into the designated floor
        for color in 0..self.state.n_colors {
            for slime in self.state.slimes[color].iter() {
                let mut current_pos: (usize, usize) = *slime;
                let target_pos: (usize, usize) = *self.state.floors[color].iter().nth(0).unwrap();
                let path: Vec<Jump> = Solver::bfs(&current_pos, &target_pos);
                self.jumps.extend(path);
            }
        }
    }
    fn bfs(&self, current_pos: &(usize, usize), target_pos: &(usize, usize)) -> Vec<Jump> {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.W]; self.H];
        let mut prev_pos: Vec<Vec<Option<(usize, usize)>>> =
            vec![vec![None; self.state.W]; self.state.H];
        queue.push_back(*current_pos);
        visited[current_pos.0][current_pos.1] = true;
        while let Some(pos) = queue.pop_front() {
            for d in 0..4 {
                let (di, dj) = D4[d];
                let ni: usize = pos.0.wrapping_add(di);
                let nj: usize = pos.1.wrapping_add(dj);
                if ni < self.state.H && nj < self.state.W && !visited[ni][nj] {
                    visited[ni][nj] = true;
                    prev_pos[ni][nj] = Some(pos);
                    queue.push_back((ni, nj));
                }
            }
        }
        let path = self.recon_path(&prev_pos, current_pos, target_pos);
        return path;
    }
    fn recon_path(
        &self,
        prev_pos: &Vec<Vec<Option<(usize, usize)>>>,
        current_pos: &(usize, usize),
        target_pos: &(usize, usize),
    ) -> Vec<Jump> {
        let mut path: Vec<Jump> = Vec::new();
        let mut current_pos: (usize, usize) = *target_pos;
        while current_pos != *current_pos {
            let prev_pos: Option<(usize, usize)> = prev_pos[current_pos.0][current_pos.1];
            path.push(Jump {
                i: prev_pos.0,
                j: prev_pos.1,
                K_remains: 1,
                d: current_pos.0 - prev_pos.0,
                l_block: 1,
            });
        }
        return path;
    }
    fn judge_all_returned(&self) -> bool {
        let mut all_returned: bool = true;
        for color in 0..self.n_colors {
            for &slime in self.state.slimes[color].iter() {
                if !self.state.floors[color].contains(&slime) {
                    all_returned = false;
                    break;
                }
            }
        }
        all_returned
    }
    fn output(&self) {
        for jump in self.jumps.iter() {
            println!(
                "{} {} {} {} {}",
                jump.i, jump.j, jump.K_remains, jump.d, jump.l_block
            );
        }
    }
}
fn main() {
    input! {n: usize, k: usize, c: [Chars; n]}
    let mut solver: Solver = Solver::new(n, k, &c);
    solver.solve();
    solver.output();
}
