use proconio::{input, marker::Chars};
use std::collections::VecDeque;
fn main() {
    input!{n: usize, s: Chars}
    let mut reversed: bool = false;
    let mut que = VecDeque::new();
    que.push_back(1);
    for i in 1..n {
        if s[i] == 'o' {
            reversed = !reversed;
        }
        if reversed {
            que.push_front(i+1);
        } else {
            que.push_back(i+1);
        }
    }
    if !reversed {
        for i in 0..n {
            print!("{} ", que[i]);
        }
        println!("");
    } else {
        for i in (0..n).rev() {
            print!("{} ", que[i]);
        }
        println!("");
    }
}
