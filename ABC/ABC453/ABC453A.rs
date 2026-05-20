use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, s: Chars}
    let mut is_streak: bool = true;
    for i in 0..n {
        if is_streak {
            if s[i] == 'o' {
                // do nothing
            } else {
                print!("{}", s[i]);
                is_streak = false;
            }
        } else {
            print!("{}", s[i]);
        }
    }
    println!("");
}
