use proconio::{input, marker::Usize1};
// ABC461C
// Q. there are gems with (color, value). Choose K gems with m different colors to maximize the total value.
// A. Greedy Algorithm | Candidates vs Replacables
fn main() {
    input!{n: usize, k: usize, m: usize, mut cv: [(Usize1, usize); n]}
    // Greedy Algorithm
    cv.sort_by(|a, b| b.1.cmp(&a.1));

    let mut color_counts: Vec<usize> = vec![0; n];
    let mut chosen_color_types: usize = 0;
    let mut total_values: usize = 0;
    for i in 0..k {
        let (color, value) = cv[i];
        if color_counts[color] == 0 {
            chosen_color_types += 1;
        }
        color_counts[color] += 1;
        total_values += value;
    }

    if chosen_color_types < m {
        // (color, value)
        let mut replacables: Vec<(usize, usize)> = Vec::new();
        let mut color_counts_rollback: Vec<usize> = color_counts.clone();
        for i in (0..k).rev() {
            if color_counts_rollback[cv[i].0] > 1 {
                color_counts_rollback[cv[i].0] -= 1;
                replacables.push(cv[i]);
            }
        }

        let mut candidates: Vec<(usize, usize)> = Vec::new();
        for i in k..n {
            // it has to add a candidate with a new color
            if color_counts[cv[i].0] == 0 {
                candidates.push(cv[i]);
                color_counts[cv[i].0] += 1;
            }
        }

        let iter_swap: usize = m - chosen_color_types;
        for i in 0..iter_swap {
            total_values += candidates[i].1;
            total_values -= replacables[i].1;
        }
    }
    println!("{}", total_values);
}