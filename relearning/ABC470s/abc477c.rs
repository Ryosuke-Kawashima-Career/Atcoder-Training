use proconio::{input, marker::Chars};
fn main() {
    input! {q: usize, s: Chars, t: Chars, queries: [(usize, usize); q]}
    let n_text: usize = s.len();
    let n_pattern: usize = t.len();
    let mut prefix_matched: Vec<usize> = vec![0; n_text + 1];
    for i in 0..n_text {
        if i + n_pattern <= n_text && s[i..i + n_pattern] == t {
            prefix_matched[i + 1] = prefix_matched[i] + 1;
        } else {
            prefix_matched[i + 1] = prefix_matched[i];
        }
    }
    for &(l, r) in queries.iter() {
        if l + n_pattern - 1 <= r {
            let count = prefix_matched[r + 1 - n_pattern] - prefix_matched[l - 1];
            if count > 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    }
}
