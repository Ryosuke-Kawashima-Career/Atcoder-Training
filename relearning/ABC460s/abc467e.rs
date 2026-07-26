use proconio::input;
// bootstrapping by fixing the initial value
fn main() {
    input! {n: usize, m: i64, a: [i64; n], b: [i64; n-1]}
    let mut sum_a: Vec<i64> = vec![0; n - 1];
    for i in 0..n - 1 {
        sum_a[i] = a[i] + a[i - 1];
        sum_a[i] %= m;
    }
    // In this phase, a[0] is fixed for bootstrapping
    let mut diff: Vec<i64> = vec![0; n - 1];
    for i in 1..n {
        if sum_a[i] < b[i - 1] {
            diff[i] = b[i - 1] - sum_a[i];
        } else {
            diff[i] = b[i - 1] + m - sum_a[i]
        }
    }
    let mut baseline: Vec<i64> = vec![0; n];
    for i in 1..n {
        baseline[i] = diff[i] + m - baseline[i - 1];
        baseline[i] %= m;
    }
    // Prepare for moving a[0]
    let mut n_odd: i64 = 0;
    let mut n_even: i64 = 0;
    for i in 0..n {
        if i % 2 == 0 {
            n_even += 1;
        } else {
            n_odd += 1;
        }
    }
    let slope: i64 = n_odd - n_even;
}
