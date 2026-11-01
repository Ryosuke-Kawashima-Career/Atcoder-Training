use proconio::{input, marker::Chars};
use std::collections::HashMap;

const MOD: u32 = 1_000_000_007;
const NONE: u32 = u32::MAX;

// A state records the last W+1 scanned cells; bit 0 is the newest.
// Horizontal adjacency is forbidden except across the row boundary.
fn enumerate_masks(w: usize, col: usize) -> Vec<u32> {
    fn dfs(bit: usize, w: usize, seam: usize, mask: u32, out: &mut Vec<u32>) {
        if bit == w + 1 {
            out.push(mask);
            return;
        }
        dfs(bit + 1, w, seam, mask, out);
        if bit == 0 || bit == seam || mask & (1 << (bit - 1)) == 0 {
            dfs(bit + 1, w, seam, mask | (1 << bit), out);
        }
    }
    // Before column 0, the boundary is between bits W-1 and W.
    let seam = if col == 0 { w } else { col };
    let mut masks = Vec::new();
    dfs(0, w, seam, 0, &mut masks);
    masks
}

fn add_mod(target: &mut u32, value: u32) {
    // Both operands are below MOD, so the sum fits in u32.
    *target += value;
    if *target >= MOD {
        *target -= MOD;
    }
}

fn count_placements(grid: &[Vec<char>]) -> u32 {
    // Minimize the frontier width by transposing when necessary.
    let (h, w) = (grid.len(), grid[0].len());
    let grid = if w > h {
        (0..w)
            .map(|j| (0..h).map(|i| grid[i][j]).collect())
            .collect::<Vec<Vec<char>>>()
    } else {
        grid.to_vec()
    };
    let (h, w) = (grid.len(), grid[0].len());
    let masks: Vec<Vec<u32>> = (0..w).map(|col| enumerate_masks(w, col)).collect();
    let keep = (1u32 << (w + 1)) - 1;
    let mut transitions = Vec::with_capacity(w);

    for col in 0..w {
        let next_col = (col + 1) % w;
        let ids: HashMap<u32, u32> = masks[next_col]
            .iter()
            .enumerate()
            .map(|(id, &mask)| (mask, id as u32))
            .collect();
        let mut forbidden = 1u32 << (w - 1); // above
        if col > 0 {
            forbidden |= 1; // left
            forbidden |= 1 << w; // upper-left
        }
        if col + 1 < w {
            forbidden |= 1 << (w - 2); // upper-right
        }
        let column_transitions = masks[col]
            .iter()
            .map(|&mask| {
                let shifted = (mask << 1) & keep;
                let empty = ids[&shifted];
                let king = if mask & forbidden == 0 {
                    ids[&(shifted | 1)]
                } else {
                    NONE
                };
                [empty, king]
            })
            .collect::<Vec<_>>();
        transitions.push(column_transitions);
    }

    let empty_id = masks[0].iter().position(|&mask| mask == 0).unwrap();
    let mut dp = vec![0u32; masks[0].len()];
    dp[empty_id] = 1;
    // State masks are no longer needed after transitions are built.
    let sizes: Vec<usize> = masks.iter().map(Vec::len).collect();
    drop(masks);
    let max_states = *sizes.iter().max().unwrap();
    dp.resize(max_states, 0);
    let mut next_dp = vec![0u32; max_states];

    for row in grid.iter().take(h) {
        for col in 0..w {
            next_dp.fill(0);
            for (id, &[empty, king]) in transitions[col].iter().enumerate() {
                let ways = dp[id];
                if ways == 0 {
                    continue;
                }
                add_mod(&mut next_dp[empty as usize], ways);
                if row[col] == '.' && king != NONE {
                    add_mod(&mut next_dp[king as usize], ways);
                }
            }
            std::mem::swap(&mut dp, &mut next_dp);
        }
    }
    let mut answer = 0;
    for &ways in &dp[..sizes[0]] {
        add_mod(&mut answer, ways);
    }
    answer
}

fn main() {
    input! {h: usize, w: usize, c: [Chars; h]}
    debug_assert!(c.iter().all(|row| row.len() == w));
    println!("{}", count_placements(&c));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute(grid: &[Vec<char>]) -> u32 {
        let (h, w) = (grid.len(), grid[0].len());
        let cells: Vec<(usize, usize)> = (0..h)
            .flat_map(|i| (0..w).map(move |j| (i, j)))
            .filter(|&(i, j)| grid[i][j] == '.')
            .collect();
        let mut answer = 0;
        for mask in 0usize..(1 << cells.len()) {
            let mut valid = true;
            for i in 0..cells.len() {
                if mask & (1 << i) == 0 {
                    continue;
                }
                for j in 0..i {
                    if mask & (1 << j) != 0
                        && cells[i].0.abs_diff(cells[j].0) <= 1
                        && cells[i].1.abs_diff(cells[j].1) <= 1
                    {
                        valid = false;
                    }
                }
            }
            if valid {
                answer += 1;
            }
        }
        answer
    }

    #[test]
    fn easy_examples() {
        assert_eq!(count_placements(&[vec!['.'; 3]]), 5);
        assert_eq!(count_placements(&vec![vec!['.'; 2]; 2]), 5);
        assert_eq!(count_placements(&[vec!['.', '#', '.']]), 4);
        assert_eq!(count_placements(&[vec!['.']]), 2);
    }

    #[test]
    fn exhaustive_small_grids() {
        for h in 1..=3 {
            for w in 1..=3 {
                for blocked in 0usize..(1 << (h * w)) {
                    let grid: Vec<Vec<char>> = (0..h)
                        .map(|i| {
                            (0..w)
                                .map(|j| {
                                    if blocked & (1 << (i * w + j)) == 0 {
                                        '.'
                                    } else {
                                        '#'
                                    }
                                })
                                .collect()
                        })
                        .collect();
                    assert_eq!(count_placements(&grid), brute(&grid), "grid={grid:?}");
                }
            }
        }
        let grid = vec![vec!['.'; 4]; 4];
        assert_eq!(count_placements(&grid), brute(&grid));
    }
}
