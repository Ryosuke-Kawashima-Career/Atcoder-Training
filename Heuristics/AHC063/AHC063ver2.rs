use proconio::input;
use std::collections::VecDeque;

const INF: usize = 1_000_000;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn to_char(self) -> char {
        match self {
            Move::Up => 'U',
            Move::Down => 'D',
            Move::Left => 'L',
            Move::Right => 'R',
        }
    }
}

// (-1, 0) Up, (1, 0) Down, (0, -1) Left, (0, 1) Right
const DIRS: [(isize, isize, Move); 4] = [
    (-1, 0, Move::Up),
    (1, 0, Move::Down),
    (0, -1, Move::Left),
    (0, 1, Move::Right),
];

struct State {
    n: usize,
    m: usize,
    grid: Vec<Vec<usize>>,
    snake: VecDeque<(usize, usize)>, // Front is head
    snake_colors: Vec<usize>,
    target_colors: Vec<usize>,
    moves: Vec<Move>,
}

impl State {
    fn new(n: usize, m: usize, grid: Vec<Vec<usize>>, target: Vec<usize>) -> Self {
        let mut snake = VecDeque::new();
        let mut snake_colors = Vec::new();
        for i in (0..=4).rev() {
            snake.push_back((i, 0));
            snake_colors.push(1);
        }
        State {
            n,
            m,
            grid,
            snake,
            snake_colors,
            target_colors: target,
            moves: Vec::new(),
        }
    }

    fn expected_target(&self) -> Option<usize> {
        let len = self.snake.len();
        if len < self.m {
            Some(self.target_colors[len])
        } else {
            None
        }
    }

    // Try to find the closest reachable food of `target_c` without eating *other* foods or hitting our body.
    fn bfs_find_food(&self, target_c: usize) -> Option<Vec<Move>> {
        let &(hx, hy) = self.snake.front().unwrap();
        let mut dist = vec![vec![INF; self.n]; self.n];
        let mut prev = vec![vec![None; self.n]; self.n];
        let mut q = VecDeque::new();

        dist[hx][hy] = 0;
        q.push_back((hx, hy));

        let mut b_count = vec![vec![0; self.n]; self.n];
        for (i, &(bx, by)) in self.snake.iter().enumerate() {
            if i > 0 {
                b_count[bx][by] += 1;
            }
        }

        let mut target_pos = None;

        while let Some((cx, cy)) = q.pop_front() {
            if cx != hx || cy != hy {
                if self.grid[cx][cy] == target_c {
                    target_pos = Some((cx, cy));
                    break;
                }
            }

            for &(dx, dy, m) in &DIRS {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if nx >= 0 && nx < self.n as isize && ny >= 0 && ny < self.n as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if b_count[nx][ny] > 0 {
                        continue;
                    }

                    let is_target = self.grid[nx][ny] == target_c;
                    let is_empty = self.grid[nx][ny] == 0;

                    if (is_empty || is_target) && dist[nx][ny] == INF {
                        dist[nx][ny] = dist[cx][cy] + 1;
                        prev[nx][ny] = Some((cx, cy, m));
                        q.push_back((nx, ny));
                    }
                }
            }
        }

        if let Some((mut tx, mut ty)) = target_pos {
            let mut path = Vec::new();
            while tx != hx || ty != hy {
                let (px, py, m) = prev[tx][ty].unwrap();
                path.push(m);
                tx = px;
                ty = py;
            }
            path.reverse();
            return Some(path);
        }
        None
    }

    // fallback: find any valid move to avoid getting boxed in.
    // Flaw fixed: we now allow stepping onto ANY food if empty isn't available!
    fn get_safe_move(&self) -> Option<Move> {
        let &(hx, hy) = self.snake.front().unwrap();
        let mut b_count = vec![vec![0; self.n]; self.n];
        for (i, &(bx, by)) in self.snake.iter().enumerate() {
            if i > 0 {
                b_count[bx][by] += 1;
            }
        }

        let mut best_move = None;
        for &(dx, dy, m) in &DIRS {
            let nx = hx as isize + dx;
            let ny = hy as isize + dy;
            if nx >= 0 && nx < self.n as isize && ny >= 0 && ny < self.n as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if b_count[nx][ny] == 0 {
                    if self.grid[nx][ny] == 0 {
                        return Some(m); // Ideal safe move
                    } else if best_move.is_none() {
                        best_move = Some(m); // Fallback to eating a wrong food rather than crashing
                    }
                }
            }
        }
        best_move
    }

    fn apply_move(&mut self, m: Move) {
        let &(hx, hy) = self.snake.front().unwrap();
        let (dx, dy) = match m {
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            Move::Left => (0, -1),
            Move::Right => (0, 1),
        };

        let nx = (hx as isize + dx) as usize;
        let ny = (hy as isize + dy) as usize;

        // Biting implementation:
        // Rule: If head lands on body at index h (1 <= h <= k-2)
        let mut bite_idx = None;
        // The tail (len-1) will move away if we don't eat, but wait: AtCoder bite rule evaluates AFTER move coords are set!
        let current_len = self.snake.len();
        for h in 1..current_len - 1 {
            let (bx, by) = self.snake[h];
            if nx == bx && ny == by {
                bite_idx = Some(h);
                break;
            }
        }

        if let Some(h) = bite_idx {
            // We bite off! Length becomes h + 1.
            // The remaining tail turns into food!
            while self.snake.len() > h {
                let (drop_x, drop_y) = self.snake.pop_back().unwrap();
                let drop_c = self.snake_colors.pop().unwrap();
                // This tail turns into food UNLESS it's the element we just stepped on!
                // Wait, the element we stepped on becomes the new head pos. The problem says:
                // "for each p=h+1...k-1, food is placed at (i'_p, j'_p)" which is just the OLD positions of the tail.
                if self.grid[drop_x][drop_y] == 0 {
                    self.grid[drop_x][drop_y] = drop_c;
                }
            }
            self.snake.push_front((nx, ny));
            // colors don't change size since we popped them
        } else {
            // Normal move or Eat
            let food_c = self.grid[nx][ny];
            let mut eating = false;

            if food_c > 0 {
                eating = true;
                self.grid[nx][ny] = 0;
                self.snake_colors.push(food_c);
            }

            self.snake.push_front((nx, ny));
            if !eating {
                self.snake.pop_back();
            }
        }

        self.moves.push(m);
    }
}

trait IsizeM {
    fn isize_m(&self) -> Move;
}
impl IsizeM for (isize, isize, Move) {
    fn isize_m(&self) -> Move {
        self.2
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        _c: usize,
        d: [usize; m],
        mut f: [[usize; n]; n]
    }

    // Initialize snake's initial cells as occupied/eaten (since it starts length 5)
    f[0][0] = 0;
    f[1][0] = 0;
    f[2][0] = 0;
    f[3][0] = 0;
    f[4][0] = 0;

    let mut state = State::new(n, m, f, d);

    while state.snake.len() < m && state.moves.len() < 100_000 {
        if let Some(target_c) = state.expected_target() {
            if let Some(path) = state.bfs_find_food(target_c) {
                // follow path
                for m in path {
                    state.apply_move(m);
                }
            } else {
                // No path to the target color.
                // In V1, we just do a random safe move.
                if let Some(safe_m) = state.get_safe_move() {
                    state.apply_move(safe_m);
                } else {
                    // Try to eat ANY food if we are stuck, wait we'd need another BFS.
                    // For now, if we're totally stuck, just break.
                    break;
                }
            }
        } else {
            break;
        }
    }

    for m in state.moves {
        println!("{}", m.to_char());
    }
}
