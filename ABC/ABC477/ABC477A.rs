use proconio::{input, marker::Chars};

fn main() {
    input! {c: char}
    if c == 'B' {
        println!("Y");
    } else if c == 'Y' {
        println!("R");
    } else {
        println!("B");
    }
}
