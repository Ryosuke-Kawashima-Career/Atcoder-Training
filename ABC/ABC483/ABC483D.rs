use proconio::input;

fn main() {
    input! {n: usize, a: [i64; n]}
    let mut prefix = vec![0_i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }

    let mut suffix_min = prefix.clone();
    let mut suffix_max = prefix.clone();
    for i in (0..n).rev() {
        suffix_min[i] = suffix_min[i].min(suffix_min[i + 1]);
        suffix_max[i] = suffix_max[i].max(suffix_max[i + 1]);
    }

    // Include the option of making no sign flip.
    let mut ans = suffix_max[0] - suffix_min[0] + 1;
    let mut left_min = 0_i64;
    let mut left_max = 0_i64;
    for i in 0..n {
        // Flipping a[i] shifts positions prefix[i + 1..=n] by -2 * a[i].
        let shift = -2 * a[i];
        let minimum = left_min.min(suffix_min[i + 1] + shift);
        let maximum = left_max.max(suffix_max[i + 1] + shift);
        ans = ans.max(maximum - minimum + 1);

        // For the next move, extend the unchanged prefix through this move.
        left_min = left_min.min(prefix[i + 1]);
        left_max = left_max.max(prefix[i + 1]);
    }
    println!("{}", ans);
}
