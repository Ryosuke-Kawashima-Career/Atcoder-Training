use proconio::{input, marker::Chars};

fn is_digit(letter: char) -> bool {
    match letter {
        '1'..='9' => true,
        _ => false,
    }
}
fn main() {
    input! {
        s: Chars
    }
    let n: usize = s.len();
    for i in 0..n {
        if is_digit(s[i]) {
            print!("{}", s[i]);
        }
    }
    println!("");
}
