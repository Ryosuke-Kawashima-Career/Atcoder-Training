use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars, t: Chars}
    let ns: usize = s.len();
    let nt: usize = t.len();
    let all_subs: usize = ns * (ns - 1) / 2;
    let count: usize = get_substrings(&s, &t);
    println!("{}", all_subs - count);
}

fn get_substrings(s: &Vec<char>, t: &Vec<char>) -> usize {
    let ns: usize = s.len();
    let nt: usize = t.len();
    let mut count: usize = 0;
    for i in (0..ns - nt + 1).rev() {
        let sub: Vec<char> = s[i..i + nt].to_vec();
        if sub == *t {
            count += (ns - nt) * (i + 1);
        }
    }
    count
}
