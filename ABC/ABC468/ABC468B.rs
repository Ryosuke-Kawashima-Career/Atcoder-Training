use proconio::{input, marker::Chars};

fn main() {
    input! {m: usize, d: usize, s: Chars}
    let mut is_supervised: Vec<bool> = vec![false; m];
    let mut guard_indexes: Vec<usize> = Vec::new();
    for i in 0..m {
        if s[i] == 'G' {
            guard_indexes.push(i);
        }
    }
    for &guard_index in guard_indexes.iter() {
        let min_index: usize = 0isize.max(guard_index as isize - d as isize) as usize;
        let max_index: usize = (m - 1).min(guard_index + d);
        for i in min_index..=max_index {
            is_supervised[i] = true;
        }
    }
    let mut ans: usize = 0;
    for &is_supervised_flag in is_supervised.iter() {
        if is_supervised_flag {
            ans += 1;
        }
    }
    println!("{}", m - ans);
}
