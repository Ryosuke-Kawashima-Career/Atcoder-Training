use proconio::input;
const MOD: usize = 998244353;
// ABC465E
// Q. 1≤x≤N that satisfy exactly one of the following three conditions.
// x is a multiple of 3. The decimal representation of x contains 3.
// Exactly three different digits are used in the decimal representation of x.
// Here, the decimal representation of an integer should not have unnecessary leading 0s.
// A. Find the number of decimals which satisfy only one condition.
fn main() {
    input! {n_str: String}
    let n_digits: Vec<usize> = n_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    // dp[is_less][is_leading_zero][mod][mask(0~9 are used?)]
    let mut dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; 1 << 10]; 3]; 2]; 2];
    dp[0][1][0][0] = 1;
    for digit_index in 0..n_digits.len() {
        let next_dp = digit_dp(n_digits[digit_index], &dp);
        dp = next_dp;
    }
    let mut ans: usize = 0;
    for mask_num in 0usize..(1usize << 10) {
        let contains_3: bool = (mask_num >> 3) & 1 == 1;
        let three_numbers_used: bool = mask_num.count_ones() == 3;
        for is_less in 0..2 {
            if !contains_3 && !three_numbers_used {
                ans += dp[is_less][0][0][mask_num];
                ans %= MOD;
            }
            if contains_3 && !three_numbers_used {
                ans += dp[is_less][0][1][mask_num];
                ans %= MOD;
                ans += dp[is_less][0][2][mask_num];
                ans %= MOD;
            }
            if !contains_3 && three_numbers_used {
                ans += dp[is_less][0][1][mask_num];
                ans %= MOD;
                ans += dp[is_less][0][2][mask_num];
                ans %= MOD;
            }
        }
    }
    println!("{}", ans % MOD);
}
fn digit_dp(limit: usize, dp: &Vec<Vec<Vec<Vec<usize>>>>) -> Vec<Vec<Vec<Vec<usize>>>> {
    /*Calculates next_dp, the dp table for the numbers with one more digit.
     limit: The upper bound for the current digit.
     dp: The dp table for the numbers with one less digit.
    */
    let mut next_dp = vec![vec![vec![vec![0; 1 << 10]; 3]; 2]; 2];
    for is_less in 0..2 {
        for is_leading_zero in 0..2 {
            for mod_3 in 0..3 {
                for mask in 0usize..(1usize << 10) {
                    if dp[is_less][is_leading_zero][mod_3][mask] == 0 {
                        continue;
                    }
                    let max_digit: usize = if is_less == 1 { 9 } else { limit };
                    for digit in 0..=max_digit {
                        let next_is_less = if is_less == 1 || digit < limit { 1 } else { 0 };
                        let next_is_leading_zero = if is_leading_zero == 1 && digit == 0 {
                            1
                        } else {
                            0
                        };
                        let next_mod_3: usize = (mod_3 + digit) % 3;
                        let next_mask: usize = if next_is_leading_zero == 1 {
                            0
                        } else {
                            mask | (1 << digit)
                        };
                        next_dp[next_is_less][next_is_leading_zero][next_mod_3][next_mask] +=
                            dp[is_less][is_leading_zero][mod_3][mask];
                        next_dp[next_is_less][next_is_leading_zero][next_mod_3][next_mask] %= MOD;
                    }
                }
            }
        }
    }
    next_dp
}
