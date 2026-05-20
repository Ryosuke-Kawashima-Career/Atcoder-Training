use proconio::input;
const INF: i64 = 1 << 60;
// Educational DP U
// Q. Divide the numbers {0..n} into some number of groups and maximize the sum of the minimums of each group.
// A. Bit Mask
/* Enumeration of sub groups
1 を引くことは、一番下の立っているビットを 0 にして、それより下のビットをすべて 1 にすることと同じである
O(3^N) ∵二項定理(∑2^n*nCk = 3^n)
 */
// Key point is N <= 16
fn main() {
    input! {n: usize, a: [[i64; n]; n]}
    let bits: usize = 1 << n;
    let mut dp: Vec<i64> = vec![0; bits];
    for mask in 0..bits {
        for i in 0..n {
            for j in i + 1..n {
                if (mask >> i) & 1 == 1 && (mask >> j) & 1 == 1 {
                    dp[mask] += a[i][j];
                }
            }
        }
    }
    // enumerate sub groups
    for mask in 0..bits {
        let mut sub_mask = (mask - 1) & mask;
        while sub_mask > 0 {
            if sub_mask == mask {
                continue;
            }
            dp[mask] = dp[mask].max(dp[sub_mask] + dp[sub_mask ^ mask]);
            // update the sub groups
            sub_mask = (sub_mask - 1) & mask;
        }
    }
    println!("{}", dp[bits - 1]);
}
