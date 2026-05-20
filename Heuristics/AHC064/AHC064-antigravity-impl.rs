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

fn find_car(state: &State, c: usize) -> Option<(usize, usize)> {
    for j in 0..state.s.len() {
        for (depth, &car) in state.s[j].iter().enumerate() {
            if car == c {
                return Some((j, depth));
            }
        }
    }
    None
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
        let cars: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        state.d[i] = cars;
    }

    let mut sequential_moves = Vec::new();

    // Turn 1: Dump all to S
    for i in 0..r_count {
        sequential_moves.push(Move {
            type_: 0,
            i,
            j: i,
            k: 10,
        });
        state.apply_move(0, i, i, 10);
    }

    let mut next_buf = 0;

    for c in 0..(10 * r_count) {
        let target_r = c / 10;
        let (j, d) = find_car(&state, c).expect("Car must be in a siding track");

        let mut remaining = d;
        let mut buffers_used = Vec::new();

        for i in 0..r_count {
            if remaining == 0 {
                break;
            }
            let buf = (next_buf + i) % r_count;
            if buf == target_r {
                continue;
            }
            let space = 15 - state.d[buf].len();
            if space > 0 {
                let take = std::cmp::min(remaining, space);
                sequential_moves.push(Move {
                    type_: 1,
                    i: buf,
                    j,
                    k: take,
                });
                state.apply_move(1, buf, j, take);
                buffers_used.push((buf, take));
                remaining -= take;
                if remaining == 0 {
                    next_buf = (buf + 1) % r_count;
                }
            }
        }

        // move car c
        sequential_moves.push(Move {
            type_: 1,
            i: target_r,
            j,
            k: 1,
        });
        state.apply_move(1, target_r, j, 1);

        // return buffers in reverse order
        for &(buf, take) in buffers_used.iter().rev() {
            sequential_moves.push(Move {
                type_: 0,
                i: buf,
                j,
                k: take,
            });
            state.apply_move(0, buf, j, take);
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
