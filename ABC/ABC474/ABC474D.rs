use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    }

    // Find an index k where Takahashi has strictly more stones than Aoki (A_k > B_k)
    let pos = (0..n).find(|&i| a[i] > b[i]);

    match pos {
        Some(k) => {
            println!("Yes");
            // Set all weights to 1, except for stone type k
            let mut w = vec![1u64; n];
            // Since sum(B) <= 10^5 * 10^9 = 10^14,
            // setting W_k = sum(B) + 1 guarantees (A_k - B_k) * W_k > sum(B) >= sum_{i != k} B_i
            let sum_b: u64 = b.iter().sum();
            w[k] = sum_b + 1;

            println!("{}", w.iter().join(" "));
        }
        None => {
            // If A_i <= B_i for all i, then for any positive W_i:
            // sum (A_i - B_i) * W_i <= 0, so Takahashi can never strictly beat Aoki.
            println!("No");
        }
    }
}
