use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, k: usize, m: usize, mut cv: [(Usize1, i64); n]}
    cv.sort_by(|gem1, gem2| gem2.1.cmp(&gem1.1));
    let mut total_value: i64 = 0;
    let mut count: usize = 0;
    let mut color_count: Vec<usize> = vec![0; n];
    for i in 0..k {
        color_count[cv[i].0] += 1;
        total_value += cv[i].1;
        if color_count[cv[i].0] == 1 {
            count += 1;
        }
    }
    if count < m {
        let mut replacables: Vec<(usize, i64)> = Vec::new();
        let mut color_count_rev: Vec<usize> = color_count.clone();
        for i in (0..k).rev() {
            if color_count_rev[cv[i].0] > 1 {
                color_count_rev[cv[i].0] -= 1;
                replacables.push((cv[i].0, cv[i].1));
            }
        }
        let mut candidates: Vec<(usize, i64)> = Vec::new();
        for i in k..n {
            if color_count[cv[i].0] == 0 {
                color_count[cv[i].0] += 1;
                candidates.push((cv[i].0, cv[i].1));
            }
        }
        let num_of_iter: usize = m - count;
        for i in 0..num_of_iter {
            total_value -= replacables[i].1;
            total_value += candidates[i].1;
        }
    }
    println!("{}", total_value);
}
