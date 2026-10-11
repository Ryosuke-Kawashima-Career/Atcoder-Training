use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, s: Chars}
    let mut streak: usize = 0;
    let mut ans: usize = 0;
    for i in 0..n {
        if s[i] == 'S' {
            ans = ans.max(streak);
            streak = 0;
        } else {
            streak += 1;
            ans = ans.max(streak);
        }
    }
    println!("{}", ans);
}
