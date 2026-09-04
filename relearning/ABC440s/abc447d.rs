use proconio::{input, marker::Chars};
// use std::cmp::Reverse;
use std::collections::BTreeSet;
fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    let mut a_indexes: BTreeSet<usize> = BTreeSet::new();
    let mut c_indexes: BTreeSet<usize> = BTreeSet::new();
    let mut b_indexes: Vec<usize> = Vec::new();
    for i in 0..n {
        if s[i] == 'A' {
            a_indexes.insert(i);
        } else if s[i] == 'B' {
            b_indexes.push(i)
        } else {
            c_indexes.insert(i);
        }
    }
    let mut ans: usize = 0;
    for &b_index in b_indexes.iter() {
        match (
            a_indexes.range(..b_index).next_back(),
            c_indexes.range(b_index..).next(),
        ) {
            (Some(&a_index), Some(&c_index)) => {
                if a_index < b_index && b_index < c_index {
                    ans += 1;
                    a_indexes.remove(&a_index);
                    c_indexes.remove(&c_index);
                }
            }
            _ => {
                continue;
            }
        }
    }
    println!("{}", ans);
}
