use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    for i1 in 0..n {
        for i2 in i1 + 1..n {
            if judge(i1, i2, &s) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn judge(i1: usize, i2: usize, s: &Vec<char>) -> bool {
    let n: usize = s.len();
    let mut diff: usize = 0;
    let mut length: usize = i2 - i1;
    if length % 2 == 0 {
        for i in 0..length / 2 {
            if s[i1 + i] != s[i2 - i - 1] {
                diff += 1
            }
        }
    } else {
        let i_mid: usize = (i1 + i2) / 2;
        for i in 0..length / 2 {
            if s[i_mid - 1 - i] != s[i_mid + i] {
                diff += 1
            }
        }
    }
    return diff <= 1;
}
