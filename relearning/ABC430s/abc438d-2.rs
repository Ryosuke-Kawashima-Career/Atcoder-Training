use proconio::input;
// abc438D
// Q. Calculate the sum of (the sum of the first k elements in a) + (the sum of the first k elements in b) + (the sum of the first k elements in c) for k = 1..n
// and output the maximum value.
// A. DP + Prefix
fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n], c: [usize; n]}
    let mut prefix_a: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        prefix_a[i + 1] = prefix_a[i] + a[i];
    }
    let mut prefix_ab: Vec<usize> = vec![0; n + 1];
    let mut prefix_abc: Vec<usize> = vec![0; n + 1];
    for i in 2..=n {
        prefix_ab[i] = (prefix_a[i - 1] + b[i - 1]).max(prefix_ab[i - 1] + b[i - 1]);
        if i >= 3 {
            prefix_abc[i] = (prefix_ab[i - 1] + c[i - 1]).max(prefix_abc[i - 1] + c[i - 1]);
        }
    }
    println!("{}", prefix_abc[n]);
}
