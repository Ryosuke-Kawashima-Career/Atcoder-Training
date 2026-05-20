use proconio::input;
const INF: usize = 1 << 60;
// ABC391 E Hierarchical Majority Vote
// Q. Does majority voting for [i..i+3], then calculate the minimum number of operations to flip the initial output to the opposite output.
// A. Split the entire string into 3-chunk segments, and calculate the minimum number of operations to flip the initial output to the opposite output.
fn main() {
    input! {n: usize, a: String}
    let n_pow_3_start = 3usize.pow(n as u32);
    let a: Vec<usize> = a.chars().map(|c| (c == '1') as usize).collect();
    // dp[phase][start][target=0 or 1] = min_ops
    // We allocate enough space.
    let mut dp: Vec<Vec<[usize; 2]>> = vec![vec![[INF; 2]; n_pow_3_start]; n + 1];
    for i in 0..n_pow_3_start {
        // if the a[i] is the same with the target, the cost is 0, else 1.
        dp[0][i][a[i]] = 0;
        dp[0][i][1 - a[i]] = 1;
    }

    // (phase, start, 01s, memo)
    let (ans0, ans1) = run_dp(n, 0, &a, &mut dp);
    // The answer is the cost to change the result. One of them is 0 (natural result), the other is the cost to flip.
    println!("{}", ans0.max(ans1));
}

fn run_dp(
    phase: usize,
    start: usize,
    a: &Vec<usize>,
    dp: &mut Vec<Vec<[usize; 2]>>,
) -> (usize, usize) {
    if phase == 0 {
        return (dp[0][start][0], dp[0][start][1]);
    }
    // Memoization check (optional but good for debugging/performance if structure wasn't perfect tree)
    if dp[phase][start][0] != INF {
        return (dp[phase][start][0], dp[phase][start][1]);
    }

    let step: usize = 3usize.pow((phase - 1) as u32);
    let mut min_ops0s = vec![INF; 3];
    let mut min_ops1s = vec![INF; 3];
    for chunk in 0..3 {
        // Recurse to phase - 1
        let (min_ops0, min_ops1) = run_dp(phase - 1, start + chunk * step, a, dp);
        min_ops0s[chunk] = min_ops0;
        min_ops1s[chunk] = min_ops1;
    }

    min_ops0s.sort();
    min_ops1s.sort();

    // To get majority 0, we need at least two 0s. Take the two cheapest costs.
    dp[phase][start][0] = min_ops0s[0] + min_ops0s[1];
    // To get majority 1, we need at least two 1s.
    dp[phase][start][1] = min_ops1s[0] + min_ops1s[1];

    return (dp[phase][start][0], dp[phase][start][1]);
}
