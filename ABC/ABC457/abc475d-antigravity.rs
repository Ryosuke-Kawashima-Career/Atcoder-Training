use proconio::input;
const INF: u64 = 3_000_000_000_000_000_000;

fn main() {
    input! {n: usize, k: u64, a: [u64; n]}
    let mut low = 0;
    let mut high = INF;
    while high - low > 1 {
        let mid = low + (high - low) / 2;
        if check(mid, &a, k) {
            low = mid;
        } else {
            high = mid;
        }
    }

    println!("{}", low);
}

fn check(target: u64, a: &[u64], k: u64) -> bool {
    let mut total_ops: u64 = 0;
    for (i, &val) in a.iter().enumerate() {
        if val < target {
            let diff = target - val;
            let step = (i + 1) as u64;
            let ops_needed = (diff + step - 1) / step;
            if total_ops
                .checked_add(ops_needed)
                .map_or(true, |sum| sum > k)
            {
                return false;
            }
            total_ops += ops_needed;
        }
    }
    total_ops <= k
}
