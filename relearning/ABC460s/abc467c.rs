use proconio::input;
// bootstrapping by fixing the initial value
fn main() {
    input! {n: usize, _m: usize, a: [i64; n], b: [i64; n-1]}
    let operations_0: usize = bootstrap(0, &a, &b);
    let operations_1: usize = bootstrap(1, &a, &b);
    let ans: usize = operations_0.min(operations_1);
    println!("{}", ans);
}

fn bootstrap(initial_value: i64, a: &Vec<i64>, b: &Vec<i64>) -> usize {
    let n = a.len();
    let mut a_cur: i64 = initial_value;
    let mut operations: usize = 0;
    if a[0] % 2 != a_cur % 2 {
        operations += 1;
    }
    for i in 1..n {
        let sum_a: i64 = (a_cur + a[i]) % 2;
        if sum_a != b[i - 1] % 2 {
            operations += 1;
            a_cur = (a[i] + 1) % 2;
        } else {
            a_cur = a[i] % 2;
        }
    }
    operations
}
