use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    let mut c_indexes: Vec<usize> = Vec::new();
    for i in 0..n {
        if s[i] == 'C' {
            c_indexes.push(i);
        }
    }
    let mut ans: usize = 0;
    for &c_index in c_indexes.iter() {
        ans += get_count(c_index, n);
    }
    println!("{}", ans);
}

fn get_count(i: usize, n: usize) -> usize {
    let left = i;
    let right = n - 1 - i;
    let mut res: usize = 1;
    let shorter: usize = left.min(right);
    res += shorter * shorter;
    return res;
}
