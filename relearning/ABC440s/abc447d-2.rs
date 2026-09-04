use proconio::{input, marker::Chars};
// ABC447D
// Q. Count the number of ABC subsequences where A < B < C
// A. Hold the three states
fn main() {
    input! {
        s: Chars,
    }

    let mut a: usize = 0; // count of available 'A's
    let mut b: usize = 0; // count of available "AB" pairs
    let mut c: usize = 0; // count of completed "ABC" triples

    for &ch in &s {
        match ch {
            'A' => a += 1,
            'B' => {
                if a > b {
                    b += 1;
                }
            }
            'C' => {
                if b > c {
                    c += 1;
                }
            }
            _ => {}
        }
    }

    println!("{}", c);
}
