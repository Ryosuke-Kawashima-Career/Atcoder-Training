use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

fn main() {
    input! { n_str: Chars }
    let digits: usize = n_str.len();

    // dp[is_less][is_leading_zero][mod3][mask]
    // is_less = 1  -> prefix already strictly less than N's prefix (free to pick 0..9)
    // is_less = 0  -> prefix exactly equal to N's prefix so far (tight)
    let mut dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; 1 << 10]; 3]; 2]; 2];
    dp[0][1][0][0] = 1;

    for i in 0..digits {
        let limit: usize = n_str[i] as usize - '0' as usize;
        dp = run_dp(limit, &dp);
    }

    let mut ans: usize = 0;
    // x can be N itself, so both is_less = 0 (tight, x == N) and is_less = 1 (x < N)
    // must be included. is_leading_zero must be 0 (excludes x == 0).
    for is_less in 0..2 {
        for mask in 0..(1usize << 10) {
            let has_three = (mask >> 3) & 1 == 1; // digit '3' was used
            let ones = mask.count_ones() as usize; // number of distinct digits used
            let exactly_three_digits = ones == 3;

            if has_three && exactly_three_digits {
                // condition B and condition C are both true -> never "exactly one"
                continue;
            } else if has_three || exactly_three_digits {
                // exactly one of B/C is true -> need condition A (mult. of 3) to be false
                ans = (ans + dp[is_less][0][1][mask]) % MOD;
                ans = (ans + dp[is_less][0][2][mask]) % MOD;
            } else {
                // neither B nor C -> need condition A to be true
                ans = (ans + dp[is_less][0][0][mask]) % MOD;
            }
        }
    }
    println!("{}", ans);
}

fn run_dp(limit: usize, dp: &Vec<Vec<Vec<Vec<usize>>>>) -> Vec<Vec<Vec<Vec<usize>>>> {
    let mut next_dp: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; 1 << 10]; 3]; 2]; 2];
    for is_less in 0..2 {
        let max_digit: usize = if is_less == 1 { 9 } else { limit };
        for is_leading_zero in 0..2 {
            for mod_3 in 0..3 {
                for mask in 0..(1usize << 10) {
                    let cur = dp[is_less][is_leading_zero][mod_3][mask];
                    if cur == 0 {
                        continue;
                    }
                    for digit in 0..=max_digit {
                        // A digit strictly below the limit makes the number free (is_less = 1);
                        // only matching the limit exactly keeps it tight.
                        let next_is_less: usize = if is_less == 1 || digit < limit { 1 } else { 0 };

                        // Leading zeros are not "real" digits: they don't enter the mask.
                        // The first nonzero digit (or any digit once past the leading zeros)
                        // is a real digit and must be added to the mask.
                        let (next_leading_zero, next_mask): (usize, usize) =
                            if is_leading_zero == 1 && digit == 0 {
                                (1, mask)
                            } else {
                                (0, mask | (1 << digit))
                            };

                        let next_mod = (mod_3 + digit) % 3;
                        next_dp[next_is_less][next_leading_zero][next_mod][next_mask] =
                            (next_dp[next_is_less][next_leading_zero][next_mod][next_mask] + cur) % MOD;
                    }
                }
            }
        }
    }
    next_dp
}
