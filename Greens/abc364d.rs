use proconio::input;
// ABC364D: Get k-th nearest values
// Q. `a` is the baseline. `b` is the query. Find the k-th nearest value to `b` in `a`.
// A. Monotonicity and Binary Search:
// If the distance is given, you can find the number of elements in `a` that are within the distance using binary search.
// Then, you can use binary search to find the k-th smallest distance.
fn main() {
    // Point 1: `k` is a cardinal count (1-indexed), so use plain `usize` instead of `Usize1`
    input! {n: usize, q: usize, mut a: [i64; n], bq: [(i64, usize); q]}

    // Point 2: `partition_point` (binary search) only works on sorted arrays!
    a.sort();

    for query in 0..q {
        let (b, k) = bq[query];
        let distance_from_kth: i64 = binary_kth(b, &a, k);
        println!("{}", distance_from_kth);
    }
}

fn binary_kth(target: i64, baseline: &[i64], kth: usize) -> i64 {
    // Point 3: To allow the answer to be 0, the low boundary must start strictly below 0
    let mut dist_low: i64 = -1;
    // The maximum possible distance between -10^8 and 10^8 is 2 * 10^8
    let mut dist_high: i64 = 300_000_000;

    while dist_high - dist_low > 1 {
        let dist_mid: i64 = dist_low + (dist_high - dist_low) / 2;
        if count_within_range(dist_mid, target, baseline) < kth {
            // This distance encapsulates fewer than `kth` elements.
            // So the answer MUST be strictly larger than `dist_mid`.
            dist_low = dist_mid;
        } else {
            // This distance encapsulates at least `kth` elements.
            // So the answer is this distance, or possibly smaller.
            dist_high = dist_mid;
        }
    }

    // `dist_high` acts as our "ok" boundary and holds the minimum valid distance.
    dist_high
}

fn count_within_range(distance: i64, target: i64, baseline: &[i64]) -> usize {
    let lower_bound: i64 = target - distance;
    let upper_bound: i64 = target + distance;
    let lower_idx: usize = baseline.partition_point(|&x| x < lower_bound);
    let upper_idx: usize = baseline.partition_point(|&x| x <= upper_bound);
    upper_idx - lower_idx
}
