use proconio::input;
use proconio::marker::Chars;
const MOD: usize = 998244353;
fn main() {
    input! {s: Chars}
    let n: usize = s.len();

    // dp[c] = subsequences without any two adjacent characters being the same
    let mut dp: Vec<usize> = vec![0; 26];
    for i in 0..n {
        let alpha: usize = s[i] as usize - b'a' as usize;
        let next_dp: Vec<usize> = run_dp(i, &s, &dp);
        dp = next_dp;
    }
    let mut ans: usize = 0;
    for c in 0..26 {
        ans += dp[c];
        ans %= MOD;
    }
    println!("{}", ans);
}

fn run_dp(index: usize, s: &Vec<char>, dp: &Vec<usize>) -> Vec<usize> {
    /*Calculates the reccurent formula
    Args:
        index: current index
        s: String
        dp: previous dp values
    Returns:
        next_dp
    */
    let n: usize = s.len();
    let mut next_dp: Vec<usize> = dp.clone();
    let cur_alpha: usize = s[index] as usize - b'a' as usize;
    for c in 0..26 {
        if c == cur_alpha {
            next_dp[cur_alpha] += 1;
            next_dp[cur_alpha] %= MOD;
            continue;
        }
        next_dp[cur_alpha] += dp[c];
        next_dp[cur_alpha] %= MOD;
    }
    return next_dp;
}
