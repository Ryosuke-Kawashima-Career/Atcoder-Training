use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
const INF: usize = 1 << 60;
// Q. You are given arrays of a and b. Find the mimnimum value of max ai * sum of bi such that i belongs to the k elements of indexes.
// A. A_r * (B_r + B_{r-1} + ... + B_1): rを決め打ちする = Constant
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, k: usize, a:[usize; n], b:[usize; n]}
        let mut zip_ab: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            zip_ab.push((a[i], b[i]));
        }
        zip_ab.sort_by_key(|x| x.0);
        let mut ans = INF;
        let mut sum_b: usize = 0;
        let mut priority_queue: BinaryHeap<usize> = BinaryHeap::new();
        // Brute Force Attack for A_r
        for r in 0..n {
            let (a_r, b_r) = zip_ab[r];
            // b_r is always added!!!
            let objective: usize = a_r * (b_r + sum_b);
            // 1 means b_r
            if priority_queue.len() == k - 1 {
                ans = ans.min(objective);
            }
            priority_queue.push(b_r);
            // b_rを足すときはsum_bがそもそも0なので，値はlower_boundになる!!!
            sum_b += b_r;
            if priority_queue.len() > k - 1 {
                let b_k = priority_queue.pop().unwrap();
                sum_b -= b_k;
            }
        }
        println!("{}", ans);
    }
}
