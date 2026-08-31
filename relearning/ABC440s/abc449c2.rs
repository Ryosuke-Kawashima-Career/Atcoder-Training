use proconio::input;
use proconio::marker::Chars;
// ABC449C
// Q. Get the number of counts such that s[i] == s[j] and l <= j - i <= r
// delta update + sliding window
fn main() {
    input! {n: usize, l: usize, r: usize, s: Chars}
    let mut count: Vec<isize> = vec![0; 26];
    let mut ans: isize = 0;
    for i in 0..n {
        // Count as a valid index
        if i >= l {
            count[s[i - l] as usize - 'a' as usize] += 1;
        }
        // i - r - 1 becomes stale.
        if i >= r + 1 {
            count[s[i - r - 1] as usize - 'a' as usize] -= 1;
        }
        ans += count[s[i] as usize - 'a' as usize];
    }
    println!("{}", ans);
}
