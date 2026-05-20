use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars, n: usize}
    for i in n..(s.len() - n) {
        print!("{}", s[i])
    }
    println!()
}
