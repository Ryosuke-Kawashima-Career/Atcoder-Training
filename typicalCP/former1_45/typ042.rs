use proconio::input;
const MOD: usize = 1_000_000_007;
fn main() {
    input!{k: usize}
    if k % 9 != 0 {
        println!("0");
        return;
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 9]; k+1];
    dp[0][0] = 1;
    for i in 1..=k {
        let upper_bound: usize = 9usize.min(i);
        for curr_mod in 0..9 {
            for digit in 1..=upper_bound {
                dp[i][(curr_mod+digit)%9] += dp[i-1][curr_mod];
                dp[i][(curr_mod+digit)%9] %= MOD
            }
        }
    }
    let ans: usize = dp[k][0] % MOD;
    println!("{}", ans);
}
