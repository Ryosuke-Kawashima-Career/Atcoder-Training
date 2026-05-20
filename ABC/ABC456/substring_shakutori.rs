use proconio::{input, marker::Chars};
const MOD: usize = 998244353;
// Q. Substring(Connected Subsegments)
// The total number of substrings of S that have no adjacent cells of the same letter.
// A. left-right-shakutori
fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    let mut ans: usize = 0;
    let mut left: usize = 0;
    while left < n {
        let mut right: usize = left + 1;
        while right < n && s[right] != s[right - 1] {
            right += 1;
        }
        let length: usize = right - left;
        let count = (length * (length + 1)) / 2;
        ans = (ans + count % MOD) % MOD;
        left = right;
    }
    println!("{}", ans % MOD);
}
