use proconio::input;

const MOD: u64 = 998244353;

fn main() {
    input! {
        n_str: String,
    }

    let n_digits: Vec<usize> = n_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let l = n_digits.len();

    // dp[is_less][is_leading_zero][mod3][mask]
    // size: 2 * 2 * 3 * 1024 = 12288
    let mut dp = vec![vec![vec![vec![0u64; 1024]; 3]; 2]; 2];

    // Base case: prefix of length 0
    // is_less = false, is_leading_zero = true, mod3 = 0, mask = 0
    dp[0][1][0][0] = 1;

    for i in 0..l {
        let mut next_dp = vec![vec![vec![vec![0u64; 1024]; 3]; 2]; 2];
        let limit = n_digits[i];

        for is_less in 0..2 {
            for is_leading_zero in 0..2 {
                for mod3 in 0..3 {
                    for mask in 0..1024 {
                        let count = dp[is_less][is_leading_zero][mod3][mask];
                        if count == 0 {
                            continue;
                        }

                        let max_d = if is_less == 1 { 9 } else { limit };
                        for d in 0..=max_d {
                            let next_is_less = if is_less == 1 || d < limit { 1 } else { 0 };

                            let (next_is_leading_zero, next_mod3, next_mask) =
                                if is_leading_zero == 1 && d == 0 {
                                    (1, 0, 0)
                                } else {
                                    (0, (mod3 + d) % 3, mask | (1 << d))
                                };

                            next_dp[next_is_less][next_is_leading_zero][next_mod3][next_mask] =
                                (next_dp[next_is_less][next_is_leading_zero][next_mod3][next_mask]
                                    + count)
                                    % MOD;
                        }
                    }
                }
            }
        }
        dp = next_dp;
    }

    let mut ans = 0u64;
    for is_less in 0..2 {
        for mod3 in 0..3 {
            for mask in 0..1024 {
                let count = dp[is_less][0][mod3][mask]; // is_leading_zero must be 0 (false)
                if count == 0 {
                    continue;
                }

                let cond1 = mod3 == 0;
                let cond2 = (mask & (1 << 3)) != 0;
                let cond3 = mask.count_ones() == 3;

                let mut match_count = 0;
                if cond1 {
                    match_count += 1;
                }
                if cond2 {
                    match_count += 1;
                }
                if cond3 {
                    match_count += 1;
                }

                if match_count == 1 {
                    ans = (ans + count) % MOD;
                }
            }
        }
    }

    println!("{}", ans);
}
