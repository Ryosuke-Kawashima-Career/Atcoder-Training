use proconio::{input, marker::Chars};

fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    let mut ans: usize = 0;
    for i in 0..n {
        if s[i] == 'C' {
            let left: usize = i;
            let right: usize = n - i - 1;
            ans += 1 + left.min(right);
        }
    }
    println!("{}", ans);
}
