use proconio::{input, fastout, marker::Chars};
// abc469c
// Q. Get the sequence of 'o' ans 'x'. Find f(k) for each k so that o >= x.
// A. Delta Update
#[fastout]
fn main() {
    input!{n: usize, s: Chars}
    // Previous Answer
    let mut f_k: usize = 0;
    for k in 0..n {
        f_k += 1;
        while f_k <= n && s[f_k - 1] == 'o' {
            f_k += 1;
        }
        println!("{}", f_k.min(n));
    }
}