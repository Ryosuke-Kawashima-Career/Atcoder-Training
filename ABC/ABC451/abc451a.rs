use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    if n % 5 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
