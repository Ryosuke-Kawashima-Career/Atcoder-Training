use proconio::{input, marker::Chars};
const MOD: u64 = 998244353;
fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    if n == 0 {
        println!("0");
        return;
    }
    let mut ans = 0u64;
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && s[j] != s[j - 1] {
            j += 1;
        }
        let len = (j - i) as u64;
        let count = (len * (len + 1) / 2) % MOD;
        ans = (ans + count) % MOD;
        i = j;
    }

    println!("{}", ans);
}
