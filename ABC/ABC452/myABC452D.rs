use proconio::{input, marker::Chars};
// ABC452D
// Q. Given two strings S and T, find the number of pairs of indices (i, j) such that S[i..j] does NOT contain T as a subsequence.
// solution: Substring DP
fn main() {
    input! {s: Chars, t: Chars}
    let n = s.len();
    let m = t.len();
    // next_indexes[i][c] is the index of the first occurrence of character c in s at or after index i.
    let mut next_indexes: Vec<Vec<usize>> = vec![vec![n; 26]; n + 1];
    for i in (0..n).rev() {
        let char_index = s[i] as usize - b'a' as usize;
        for c in 0..26 {
            if c == char_index {
                next_indexes[i][c] = i;
            } else {
                next_indexes[i][c] = next_indexes[i + 1][c];
            }
        }
    }
    let mut ans: usize = 0;
    for left in 0..n {
        let mut found_all_t: bool = true;
        let mut next_index: usize = left;
        for it in 0..m {
            next_index = next_indexes[next_index][t[it] as usize - b'a' as usize];
            if next_index == n {
                found_all_t = false;
                break;
            }
            next_index += 1;
        }
        if found_all_t {
            ans += next_index - left - 1;
        } else {
            ans += n - left;
        }
    }
    println!("{}", ans);
}
