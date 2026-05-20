use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Belt {
    cells: Vec<(usize, usize)>,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n_line = lines.next();
    if n_line.is_none() {
        return;
    }
    let n: usize = n_line.unwrap().unwrap().trim().parse().unwrap();
    let mut grid = vec![vec![999; n]; n];
    let mut box_pos = vec![(0, 0); n * n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..n {
            grid[i][j] = row[j];
            box_pos[row[j]] = (i, j);
        }
    }

    let exit_r = 0;
    let exit_c = n / 2;
    let mut removed = vec![false; n * n];
    let mut next_to_remove = 0;

    let mut belts = vec![];

    for k in 0..(n / 2) {
        let mut cells = vec![];
        let r1 = 2 * k;
        let r2 = 2 * k + 1;
        for j in 0..n {
            cells.push((r1, j));
        }
        for j in (0..n).rev() {
            cells.push((r2, j));
        }
        belts.push(Belt { cells });
    }

    for k in 0..(n / 2) {
        let mut cells = vec![];
        let c1 = 2 * k;
        let c2 = 2 * k + 1;
        for i in 0..n {
            cells.push((i, c1));
        }
        for i in (0..n).rev() {
            cells.push((i, c2));
        }
        belts.push(Belt { cells });
    }

    let mut operations = vec![];

    let mut apply_shift = |belt_id: usize,
                           d: isize,
                           grid: &mut Vec<Vec<usize>>,
                           box_pos: &mut Vec<(usize, usize)>,
                           removed: &mut Vec<bool>,
                           next_to_remove: &mut usize,
                           operations: &mut Vec<(usize, isize)>| {
        let steps = d.abs();
        let dir = if d > 0 { 1 } else { -1 };
        let belt = &belts[belt_id];
        let len = belt.cells.len();

        for _ in 0..steps {
            operations.push((belt_id, dir));
            let mut new_vals = vec![0; len];
            for i in 0..len {
                let prev_idx = (i as isize - dir + len as isize) % len as isize;
                let (pr, pc) = belt.cells[prev_idx as usize];
                new_vals[i] = grid[pr][pc];
            }
            for i in 0..len {
                let (r, c) = belt.cells[i];
                grid[r][c] = new_vals[i];
                if grid[r][c] != 999 {
                    box_pos[grid[r][c]] = (r, c);
                }
            }

            while *next_to_remove < n * n && grid[exit_r][exit_c] == *next_to_remove {
                removed[*next_to_remove] = true;
                grid[exit_r][exit_c] = 999;
                *next_to_remove += 1;
            }
        }
    };

    while next_to_remove < n * n && grid[exit_r][exit_c] == next_to_remove {
        removed[next_to_remove] = true;
        grid[exit_r][exit_c] = 999;
        next_to_remove += 1;
    }

    for target in 0..(n * n) {
        if removed[target] {
            continue;
        }

        let (r, c) = box_pos[target];

        let mut d_a1 = 0isize;
        let mut target_a1 = (r, c);
        if c != 10 && c != 11 {
            let h_belt_id = r / 2;
            let belt = &belts[h_belt_id];
            let len = belt.cells.len();
            let mut current_idx = 0;
            for i in 0..len {
                if belt.cells[i] == (r, c) {
                    current_idx = i;
                    break;
                }
            }
            let mut best_d = 999isize;
            for i in 0..len {
                let (_, bc) = belt.cells[i];
                if bc == 10 || bc == 11 {
                    let d = (i as isize - current_idx as isize + len as isize) % len as isize;
                    let d = if d > (len as isize / 2) {
                        d - len as isize
                    } else {
                        d
                    };
                    if d.abs() < best_d.abs() {
                        best_d = d;
                        target_a1 = belt.cells[i];
                    }
                }
            }
            d_a1 = best_d;
        }

        let (r_a1, c_a1) = target_a1;
        let v_belt_id = 10 + 5;
        let v_belt = &belts[v_belt_id];
        let v_len = v_belt.cells.len();
        let mut v_current_idx = 0;
        for i in 0..v_len {
            if v_belt.cells[i] == (r_a1, c_a1) {
                v_current_idx = i;
                break;
            }
        }
        let mut v_target_idx = 0;
        for i in 0..v_len {
            if v_belt.cells[i] == (0, 10) {
                v_target_idx = i;
                break;
            }
        }
        let d_a2 =
            (v_target_idx as isize - v_current_idx as isize + v_len as isize) % v_len as isize;
        let d_a2 = if d_a2 > (v_len as isize / 2) {
            d_a2 - v_len as isize
        } else {
            d_a2
        };
        let cost_a = d_a1.abs() + d_a2.abs();

        let mut d_b1 = 0isize;
        let mut target_b1 = (r, c);
        if r != 0 && r != 1 {
            let v_belt_id = 10 + c / 2;
            let belt = &belts[v_belt_id];
            let len = belt.cells.len();
            let mut current_idx = 0;
            for i in 0..len {
                if belt.cells[i] == (r, c) {
                    current_idx = i;
                    break;
                }
            }
            let mut best_d = 999isize;
            for i in 0..len {
                let (br, _) = belt.cells[i];
                if br == 0 || br == 1 {
                    let d = (i as isize - current_idx as isize + len as isize) % len as isize;
                    let d = if d > (len as isize / 2) {
                        d - len as isize
                    } else {
                        d
                    };
                    if d.abs() < best_d.abs() {
                        best_d = d;
                        target_b1 = belt.cells[i];
                    }
                }
            }
            d_b1 = best_d;
        }

        let (r_b1, c_b1) = target_b1;
        let h_belt_id = 0;
        let h_belt = &belts[h_belt_id];
        let h_len = h_belt.cells.len();
        let mut h_current_idx = 0;
        for i in 0..h_len {
            if h_belt.cells[i] == (r_b1, c_b1) {
                h_current_idx = i;
                break;
            }
        }
        let mut h_target_idx = 0;
        for i in 0..h_len {
            if h_belt.cells[i] == (0, 10) {
                h_target_idx = i;
                break;
            }
        }
        let d_b2 =
            (h_target_idx as isize - h_current_idx as isize + h_len as isize) % h_len as isize;
        let d_b2 = if d_b2 > (h_len as isize / 2) {
            d_b2 - h_len as isize
        } else {
            d_b2
        };
        let cost_b = d_b1.abs() + d_b2.abs();

        if cost_a <= cost_b {
            if d_a1 != 0 {
                apply_shift(
                    r / 2,
                    d_a1,
                    &mut grid,
                    &mut box_pos,
                    &mut removed,
                    &mut next_to_remove,
                    &mut operations,
                );
            }
            if !removed[target] && d_a2 != 0 {
                apply_shift(
                    10 + 5,
                    d_a2,
                    &mut grid,
                    &mut box_pos,
                    &mut removed,
                    &mut next_to_remove,
                    &mut operations,
                );
            }
        } else {
            if d_b1 != 0 {
                apply_shift(
                    10 + c / 2,
                    d_b1,
                    &mut grid,
                    &mut box_pos,
                    &mut removed,
                    &mut next_to_remove,
                    &mut operations,
                );
            }
            if !removed[target] && d_b2 != 0 {
                apply_shift(
                    0,
                    d_b2,
                    &mut grid,
                    &mut box_pos,
                    &mut removed,
                    &mut next_to_remove,
                    &mut operations,
                );
            }
        }
    }

    // Output
    println!("{}", belts.len());
    for belt in &belts {
        print!("{}", belt.cells.len());
        for &(r, c) in &belt.cells {
            print!(" {} {}", r, c);
        }
        println!();
    }
    println!("{}", operations.len());
    for &(id, d) in &operations {
        println!("{} {}", id, d);
    }
}
