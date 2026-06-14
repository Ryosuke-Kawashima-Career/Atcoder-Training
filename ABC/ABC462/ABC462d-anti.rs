use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        st: [(i64, i64); n],
    }

    // The maximum possible value for S_i and T_i is 10^6.
    let max_t = 1_000_000;
    let mut diff = vec![0i64; max_t + 5];

    for &(s, t) in &st {
        let l = s;
        let r = t - d;
        if l <= r {
            // This suspect can be a culprit for any crime starting at x in [l, r].
            // Add 1 to the interval [l, r].
            diff[l as usize] += 1;
            diff[(r + 1) as usize] -= 1;
        }
    }

    let mut ans = 0u64;
    let mut current_suspects = 0i64;
    for x in 1..=max_t {
        current_suspects += diff[x];
        if current_suspects >= 2 {
            let count = current_suspects as u64;
            ans += count * (count - 1) / 2;
        }
    }

    println!("{}", ans);
}
