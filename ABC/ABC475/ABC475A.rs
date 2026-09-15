use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}
    let n = s.len();
    for i in 0..n - 1 {
        print!("{}o", s[i]);
    }
    println!("{}", s[n - 1]);
}
