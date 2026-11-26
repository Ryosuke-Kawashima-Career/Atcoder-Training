use proconio::input;
// Typical90 Problem 066: Various Arrays
// Find the expectation of inverses
// Key insight: Linearity of Expectation
// E[inversions] = sum_{i < j} P(a_i > a_j)
fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut expectations: f64 = 0.0;

    // Iterate over every pair of indices (i, j) with 0 <= i < j < n.
    // For each pair, let (l_i, r_i) = lr[i] and (l_j, r_j) = lr[j].
    for i in 0..n {
        for j in i + 1..n {
            let (left_i, right_i) = lr[i];
            let (left_j, right_j) = lr[j];
            // the sum of expectations is the total expectations
            // the total expectations is the sum of probabilities
            let pairs: usize = (right_i - left_i + 1) * (right_j - left_j + 1);
            let mut inverses: usize = 0;
            for x in left_i..=right_i {
                for y in left_j..=right_j {
                    if x > y {
                        inverses += 1;
                    }
                }
            }
            expectations += (inverses as f64) / (pairs as f64);
        }
    }
    println!("{:.10}", expectations);
}
