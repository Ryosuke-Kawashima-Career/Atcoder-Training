use proconio::{input, marker::Chars};
const MOD: usize = 998244353;

fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    let mut ans: usize = n;
    let mut left: usize = 1;
    for i in 0..n {
        let mut right = left.max(i + 1);
        while right < n && s[right] != s[right - 1] {
            right += 1;
        }
        let add: usize = right - i - 1;
        ans += add;
        ans %= MOD;
        left = right;
    }

    println!("{}", ans % MOD);
}
