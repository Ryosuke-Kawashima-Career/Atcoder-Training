use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, s: Chars, t: Chars}
    let mut is_ok: bool = true;
    for i in 0..n {
        if t[i] != '*' {
            if s[i] != t[i] {
                is_ok = false;
                break;
            }
        }
    }
    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
