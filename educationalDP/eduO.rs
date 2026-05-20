use proconio::input;
const MOD: usize = 1_000_000_007;
fn main() {
    input! {n: usize, a: [[usize; n]; n]}
    let bits: usize = 1 << n;
    // Bit DP: dp[mask]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; bits]; n + 1];
    dp[0][0] = 1;
    for person in 1..=n {
        for paired_mask in 0..bits {
            let paired_num: usize = paired_mask.count_ones() as usize;
            if paired_num != person - 1 {
                continue;
            }
            for another in 0..n {
                if paired_mask >> another & 1 == 0 && a[person - 1][another] == 1 {
                    let next_mask = paired_mask | (1 << another);
                    dp[person][next_mask] += dp[person - 1][paired_mask];
                    dp[person][next_mask] %= MOD;
                }
            }
        }
    }
    println!("{}", dp[n][bits - 1] % MOD);
}
