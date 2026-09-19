use proconio::{input, marker::Chars};

fn main() {
    input! { mut s: Chars }
    let n: usize = s.len();
    if s[n - 1] == 'e' {
        s.push('r');
    } else {
        s.extend(&['e', 'r']);
    }
    let ans: String = s.into_iter().collect();
    println!("{}", ans);
}
