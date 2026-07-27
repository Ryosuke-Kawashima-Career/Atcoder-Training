use proconio::input;
// find maximum subarray sum
// subsequence (continuous) vs subsequence
fn main() {
    input! {n: usize, a: [i64; n]}
    // the maximum subarray sum whose end is index(i).
    // if max_endings[i] == a[i], i is the start!!!
    let mut max_endings: Vec<i64> = vec![i64::MIN; n];
    let mut max_subarray_sum: i64 = i64::MIN;
    max_endings[0] = a[0];
    max_subarray_sum = max_endings[0];
    for i in 1..n {
        // the case of i being  the end vs the case of i being the start
        max_endings[i] = (max_endings[i - 1] + a[i]).max(a[i]);
        max_subarray_sum = max_subarray_sum.max(max_endings[i]);
    }
    println!("{}", max_subarray_sum);
}
