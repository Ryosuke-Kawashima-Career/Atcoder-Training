use proconio::input;
use std::collections::HashMap;
// ABC371E
// Q. Σ Σ(i=j<n) f(i, j) such that f(i, j) = the number of kinds of integers in {a_i, a_{i+1}, ..., a_j}.
// A. Contribution Method: 主客転倒法
// A: Calculate the contribution of each index k to the answer.
fn main() {
    input! {n: usize, a: [i64; n]}
    let mut last_indexes: HashMap<i64, isize> = HashMap::new();
    let mut contributions: Vec<isize> = vec![0isize; n];
    for i in 0..n {
        let current_idx: isize = i as isize;
        if let Some(&prev_idx) = last_indexes.get(&a[i]) {
            // 各indexの貢献度= left * right
            let left_choices: isize = current_idx - prev_idx;
            let right_choices: isize = n as isize - current_idx;
            contributions[i] = left_choices * right_choices;
        } else {
            let left_choices: isize = current_idx + 1;
            let right_choices: isize = n as isize - current_idx;
            contributions[i] = left_choices * right_choices;
        }
        last_indexes.insert(a[i], current_idx);
    }
    let sum_contributions: isize = contributions.iter().sum();
    println!("{}", sum_contributions);
}
