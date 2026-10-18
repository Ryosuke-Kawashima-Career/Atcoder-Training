use proconio::input;
// ABC480D
// Q. Prefix sum in range [l,r]
// Arrays are defined as follows: if a = [1, 10, 100], b = [1, 1, 10, 1, 10, 100]
// A. 2 layers of prefix sums
fn main() {
    input! {n: usize, q: usize, a: [i64; n], queries: [(usize, usize); q]}
    let mut pref_a: Vec<i64> = vec![0i64; n + 1];
    for i in 0..n {
        pref_a[i + 1] = pref_a[i] + a[i];
    }
    // pref_block[i] := sum of elements in the first i blocks
    // len[i] := total count of elements in the first i blocks
    let mut pref_block: Vec<i64> = vec![0i64; n + 1];
    let mut len: Vec<usize> = vec![0usize; n + 1];
    for i in 1..=n {
        pref_block[i] = pref_block[i - 1] + pref_a[i];
        len[i] = len[i - 1] + i;
    }
    let get_pref_sum = |k: usize| -> i64 {
        // Largest block index i such that len[i] <= k
        // Partition point returns the first element returning false := in order to get true, -1 is needed
        let blocks: usize = len.partition_point(|&x| x <= k) - 1;
        let rem: usize = k - len[blocks];
        pref_block[blocks] + pref_a[rem]
    };
    for (l, r) in queries {
        let ans = get_pref_sum(r) - get_pref_sum(l - 1);
        println!("{}", ans);
    }
}
