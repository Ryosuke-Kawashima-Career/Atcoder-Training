use proconio::input;
const INF: usize = 1 << 60;
// ABC391 E Hierarchical Majority Vote
// Q. Does majority voting for [i..i+3], then calculate the minimum number of operations to flip the initial output to the opposite output.
// A. Iterative DP
fn main() {
    input! {n: usize, a: String}
    let n_pow_3_start = 3usize.pow(n as u32);
    let a: Vec<usize> = a.chars().map(|c| (c == '1') as usize).collect();
    // dp[phase][index] = (value, count)
    // We allocate enough space.
    let mut dp: Vec<Vec<(usize, usize)>> = vec![vec![(INF, 0); n_pow_3_start]; n + 1];
    for i in 0..n_pow_3_start {
        dp[0][i] = (a[i], 1);
    }

    for phase in 1..=n {
        let current_limit = 3usize.pow((n - phase) as u32);
        for i in 0..current_limit {
            let prev_i_0 = 3 * i;
            let prev_i_1 = 3 * i + 1;
            let prev_i_2 = 3 * i + 2;
            let (value0, count0) = dp[phase - 1][prev_i_0];
            let (value1, count1) = dp[phase - 1][prev_i_1];
            let (value2, count2) = dp[phase - 1][prev_i_2];

            // value: the value of the majority vote
            // count: the minimum number of operations to flip the initial output to the opposite output
            if value0 == value1 && value1 == value2 {
                dp[phase][i] = (
                    value0,
                    count0 + count1 + count2 - count0.max(count1).max(count2),
                );
            } else if value0 == value1 {
                dp[phase][i] = (value0, count0.min(count1));
            } else if value0 == value2 {
                dp[phase][i] = (value0, count0.min(count2));
            } else {
                // value1 == value2
                dp[phase][i] = (value1, count1.min(count2));
            }
        }
    }

    let ans = dp[n][0];
    println!("{}", ans.1);
}
