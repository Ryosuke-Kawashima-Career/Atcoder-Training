use proconio::{input, marker::Usize1};
fn main() {
    input! {n: usize, k: usize, m_colors: usize, cv: [(usize, i64); n]}
    let mut color_values: Vec<Vec<i64>> = vec![vec![0]; n];
    for &(color, value) in cv.iter() {
        color_values[color].push(value);
    }
    for color in 0..n {
        color_values[color].sort_by(|a, b| b.cmp(&a));
    }
    color_values.sort_by(|a, b| b[0].cmp(&a[0]));
    let mut total_value: i64 = 0;
    for color in 0..m_colors {
        total_value += color_values[color][0];
    }
    let remains: usize = k - m_colors;
    let mut candidates: Vec<i64> = Vec::new();
    for color in 0..n {
        if color < m_colors {
            for i in 1..color_values[color].len() {
                if color_values[color][i] == 0 {
                    break;
                }
                candidates.push(color_values[color][i]);
            }
        } else {
            for i in 0..color_values[color].len() {
                if color_values[color][i] == 0 {
                    break;
                }
                candidates.push(color_values[color][i]);
            }
        }
    }
    candidates.sort_by(|a, b| b.cmp(&a));
    for i in 0..remains {
        total_value += candidates[i];
    }
    println!("{}", total_value);
}
