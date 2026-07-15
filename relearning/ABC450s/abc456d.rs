use proconio::input;
use proconio::marker::Chars;
const MOD: usize = 998244353;
fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    let mut last_indexes: Vec<Vec<usize>> = vec![vec![n; n + 1]; 26];
    for i in (0..n).rev() {
        let alpha: usize = s[i] as usize - b'a' as usize;
        last_indexes[alpha] = i;
        for c in 0..26 {
            last_indexes[c][i] = last_indexes[c][i].min(last_indexes[c][i + 1]);
        }
    }

    // dp[c] = subsequences without any two adjacent characters being the same
    let mut dp: Vec<usize> = vec![0; 26];
    for i in 0..n {
        let alpha: usize = s[i] as usize - b'a' as usize;
        let next_dp: Vec<usize> = run_dp(i, &s, &last_indexes, &dp);
        dp = next_dp;
    }
    let mut ans: usize = 0;
    for c in 0..26 {
        ans += dp[c];
        ans %= MOD;
    }
    println!("{}", ans);
}

fn run_dp(
    index: usize,
    s: &Vec<char>,
    last_indexes: &Vec<Vec<usize>>,
    dp: &Vec<usize>,
) -> Vec<usize> {
    /*Calculates the reccurent formula
    Args:
        index: current index
        s: String
        last_indexes: next indexes of each alphabet
        dp: previous dp values
    Returns:
        next_dp
    */
    let n: usize = s.len();
    let mut next_dp: Vec<usize> = dp.clone();
    let cur_alpha: usize = s[index] as usize - b'a' as usize;
    for c in 0..26 {
        let next_index: usize = last_indexes[c][index];
    }
}
