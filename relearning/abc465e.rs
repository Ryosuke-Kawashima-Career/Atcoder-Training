use proconio::{input, marker::Chars};
const MOD: usize = 998244353;
fn main() {
    input!{n_str: Chars}
    // dp[is_less][is_leading_zero][mod 3][mask]
    let mut dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; 1 << 10]; 3]; 2]; 2];
    dp[0][1][0][0] = 1;
    let digits: usize = n_str.len();
    for digit in 0..digits {
        let limit: usize = n_str[digit] as usize - '0' as usize;
        let next_dp: Vec<Vec<Vec<Vec<usize>>>> = run_dp(limit, &dp);
        dp = next_dp;
    }
    let mut ans: usize = 0;
    for mask in 0..(1usize << 10) {
        let ones: usize = mask.count_ones() as usize;
        if ones == 3 {
            if (mask >> 3) & 1 == 1 {
                continue;
            } else {
                ans += dp[1][0][1][mask];
                ans %= MOD;
                ans += dp[1][0][2][mask];
                ans %= MOD;
            }
        } else {
            if (mask >> 3) & 1 == 1 {
                ans += dp[1][0][1][mask];
                ans %= MOD;
                ans += dp[1][0][2][mask];
                ans %= MOD;
            } else {
                ans += dp[1][0][0][mask];
                ans %= MOD;
            }
        }
    }
    println!("{}", ans);
}

fn run_dp(limit: usize, dp: &Vec<Vec<Vec<Vec<usize>>>>) -> Vec<Vec<Vec<Vec<usize>>>> {
    let mut next_dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; 1 << 10]; 3]; 2]; 2];
    for is_less in 0..2 {
        for is_leading_zero in 0..2 {
            for mod_3 in 0..3 {
                for mask in 0..(1usize << 10) {
                    let max_digit: usize = if is_less == 1 {
                        9
                    } else {
                        limit
                    };
                    for digit in 0..=max_digit {
                        if is_leading_zero == 1 {
                            if digit == 0 {
                                next_dp[is_less][1][(mod_3+digit)%3][mask] += dp[is_less][1][mod_3][mask];
                                next_dp[is_less][1][(mod_3+digit)%3][mask] %= MOD;
                            } else {
                                next_dp[is_less][0][(mod_3+digit)%3][mask] += dp[is_less][1][mod_3][mask];
                                next_dp[is_less][0][(mod_3+digit)%3][mask] %= MOD;
                            }
                        } else {
                            next_dp[is_less][0][(mod_3+digit)%3][mask] += dp[is_less][0][mod_3][mask];
                            next_dp[is_less][0][(mod_3+digit)%3][mask] %= MOD;
                        }
                    }
                }
            }
        }
    }
    return next_dp;
}
