use proconio::{input, marker::Chars};
const MOD: u64 = 998244353;
// 部分列の数え上げだから多分DPでいけるのでは?
fn main() {
    input! {s: Chars}
    let mut dp = [0u64; 3];

    for c in s {
        let idx = (c as u8 - b'a') as usize;
        let others_sum = (dp[(idx + 1) % 3] + dp[(idx + 2) % 3]) % MOD;
        dp[idx] = (dp[idx] + others_sum + 1) % MOD;
    }
    let ans = (dp[0] + dp[1] + dp[2]) % MOD;
    println!("{}", ans);
}
