use proconio::{input, marker::Chars};
// const INF: i64 = 1 << 30;
fn main() {
    input! {s: Chars, t: Chars}
    let n: usize = s.len();
    let m: usize = t.len();
    let mut next_indexes: Vec<Vec<usize>> = vec![vec![n; 26]; n + 1];
    for i in (0..n).rev() {
        for c in 0..26 {
            if s[i] as u8 == c as u8 + b'a' {
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
        for state_t in 0..m {
            next_index = next_indexes[next_index][t[state_t] as usize - b'a' as usize];
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
