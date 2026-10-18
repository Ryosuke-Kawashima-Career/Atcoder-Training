use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
    }

    let total_sum: u64 = a.iter().sum();
    let w = total_sum / (m as u64); // Target cake size per person

    let num_states = 1usize << n;
    let mut sum = vec![0u64; num_states];
    let mut dp = vec![0usize; num_states];

    for mask in 1..num_states {
        // Extract the lowest set bit to compute `sum[mask]` in O(1)
        let lowbit = mask & mask.wrapping_neg();
        let idx = lowbit.trailing_zeros() as usize;
        sum[mask] = sum[mask ^ lowbit] + a[idx];

        // Find the maximum valid prefix count among all predecessors
        let mut best_prev = 0;
        let mut sub = mask;
        while sub > 0 {
            let bit = sub & sub.wrapping_neg();
            best_prev = best_prev.max(dp[mask ^ bit]);
            sub ^= bit;
        }

        dp[mask] = best_prev + usize::from(sum[mask] % w == 0);
    }

    let max_components = dp[num_states - 1];
    println!("{}", m - max_components);
}
