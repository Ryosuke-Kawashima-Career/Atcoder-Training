use proconio::input;
const INF: i64 = 1 << 60;
fn main() {
    input! {n: usize, a: [i64; n]}
    // min_number_of_the_top = dp[length of the sequence]
    let mut min_top: Vec<i64> = vec![INF; n + 1];
    min_top[0] = -1;
    let mut max_length: usize = 0;
    for i in 0..n {
        let length: usize = min_top.partition_point(|&x| x < a[i]);
        max_length = max_length.max(length);
        min_top[length] = a[i];
    }
    let lis_length: usize = min_top.partition_point(|&x| x != INF);
    println!("{}", lis_length.saturating_sub(1));
    println!("{}", max_length);
}
