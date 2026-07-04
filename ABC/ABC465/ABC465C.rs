use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut q = VecDeque::new();
    let mut reversed = false;

    for k in 1..=n {
        if s[k - 1] == 'o' {
            reversed = !reversed;
            if reversed {
                q.push_back(k);
            } else {
                q.push_front(k);
            }
        } else {
            if reversed {
                q.push_front(k);
            } else {
                q.push_back(k);
            }
        }
    }

    if reversed {
        for (i, &x) in q.iter().rev().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", x);
        }
    } else {
        for (i, &x) in q.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", x);
        }
    }
    println!("");
}
