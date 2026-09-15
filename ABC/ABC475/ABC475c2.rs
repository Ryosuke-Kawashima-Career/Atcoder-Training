use proconio::{input, marker::Usize1};

// get the longest transitions within the length of l.
fn main() {
    input! {n: usize, start: Usize1, limit: i64, a: [i64; n-1]}
    let mut prefix: Vec<i64> = vec![0; n];
    for i in 1..n {
        prefix[i] = prefix[i - 1] + a[i - 1];
    }
    let mut ans: usize = 1;
    for l in 0..=start {
        for r in start..n {
            let dist_left: i64 = prefix[start] - prefix[l];
            let dist_right: i64 = prefix[r] - prefix[start];
            let dist_min: i64 = (2 * dist_left + dist_right).min(2 * dist_right + dist_left);
            if dist_min <= limit {
                ans = ans.max(r - l + 1);
            }
        }
    }
    println!("{}", ans);
}
