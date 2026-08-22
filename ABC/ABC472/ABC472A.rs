use proconio::{input, marker::Chars};

fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    for i in 0..n {
        if s[i] == 'A' {
            print!(".");
        } else {
            print!("{}", s[i]);
        }
    }
    println!("");
}
