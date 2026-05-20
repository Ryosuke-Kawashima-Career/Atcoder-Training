use proconio::{input, marker::Chars};
const MOD: usize = 998244353;
// 尺取り法では？
// ABC456C

fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    let mut cur_char: char = 'x';
    let mut count: usize = 0;
    let mut left: usize = 0;
    while left < n {
        let mut right: usize = left;
        while right < n && s[right] != cur_char {
            cur_char = s[right];
            right += 1;
        }
        count += right - left;
        if left == right {
            left = right + 1;
        } else {
            left = right;
        }
    }
    println!("{}", count);
}
