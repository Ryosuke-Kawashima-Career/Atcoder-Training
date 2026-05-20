use itertools::Itertools;
use proconio::input;
const INF: usize = 1 << 60;
// Q. You are given arrays of a and b. Find the mimnimum value of max ai * sum of bi such that i belongs to the k elements of indexes.
// A.
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, k: usize, a:[usize; n], b:[usize; n]}
        let mut ans = INF;
        for comb in (0..n).combinations(k) {
            let mut sum_b = 0;
            let mut max_a = 0;
            for i in 0..k {
                max_a = max_a.max(a[comb[i]]);
                sum_b += b[comb[i]];
            }
            ans = ans.min(max_a * sum_b);
        }
        println!("{}", ans);
    }
}
