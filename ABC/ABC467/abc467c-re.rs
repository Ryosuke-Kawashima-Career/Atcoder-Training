use proconio::input;
// ABC467C
// Q. Make the mode of a[i] + a[i+1] the same with b[i]. Find the minimum operations to achieve this.
// A. Decide the initial value -> Bootstrapping
// Note that m is equal to 2.
fn main() {
    input! {n: usize, _m: usize, a: [usize; n], b: [usize; n-1]}
    let mut cost_0: Vec<usize> = vec![0; n];
    let mut cost_1: Vec<usize> = vec![0; n];
    if a[0] % 2 == 0 {
        cost_1[0] = 1;
    } else {
        cost_0[0] = 1;
    }
    run_step(0, &mut cost_0, &a, &b);
    run_step(1, &mut cost_1, &a, &b);
    let ans: usize = cost_0[n - 1].min(cost_1[n - 1]);
    println!("{}", ans);
}

fn run_step(a0: usize, cost: &mut Vec<usize>, a: &[usize], b: &[usize]) {
    /* Calculate the cost by setting the initial value: the Chain reaction */
    let n: usize = cost.len();
    let mut a_cur: usize = a0;
    for i in 1..n {
        if (a_cur + a[i]) % 2 == b[i - 1] % 2 {
            cost[i] = cost[i - 1];
            a_cur = a[i] % 2;
        } else {
            cost[i] = cost[i - 1] + 1;
            a_cur = (a[i] + 1) % 2;
        }
    }
}
