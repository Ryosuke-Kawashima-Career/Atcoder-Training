use proconio::input;
const MOD: i64 = 1_000_000_000;
// ABC401C - K-Fibonacci
// Q. K-Fibonacci
// A. Sliding Window
fn main() {
    input! {n: usize, k: usize}
    let mut a: Vec<i64> = vec![1; n + 1];
    let mut sum: Vec<i64> = (1..=n as i64 + 1).collect();
    for i in k..=n {
        // sumがk-fibonacciの和を表す
        a[i] = sum[i - 1];
        // iとi-kがスライドしていくイメージ
        // Use rem_euclid to handle negative results correctly
        sum[i] = (sum[i - 1] + a[i] - a[i - k]).rem_euclid(MOD);
    }
    println!("{}", a[n] % MOD);
}
