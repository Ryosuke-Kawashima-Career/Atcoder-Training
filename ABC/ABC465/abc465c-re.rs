use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, s: Chars}
    let mut reversed: bool = false;
    let mut deque = std::collections::VecDeque::new();
    for i in 0..n {
        if reversed {
            deque.push_back(i + 1);
        } else {
            deque.push_front(i + 1);
        }
        if s[i] == 'o' {
            reversed = !reversed;
        }
    }
    if !reversed {
        for i in 0..n {
            print!("{} ", deque[n - 1 - i]);
        }
    } else {
        for i in 0..n {
            print!("{} ", deque[i]);
        }
    }
    println!("");
}
