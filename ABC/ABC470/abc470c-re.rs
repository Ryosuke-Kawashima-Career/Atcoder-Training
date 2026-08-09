use proconio::{input, marker::Usize1};
// ABC470C
// Q. 1 x: Increase the value of Ax by 1.
// 2: For each i=1,2,…,N, if Ai≥1, decrease the value of Ai by 1.
// A. Store the indexes which changed its status.
fn main() {
    input! {n: usize, q: usize}
    let mut a: Vec<usize> = vec![0; n];
    // Manage Status
    let mut more_then_ones: Vec<usize> = Vec::new();
    let mut nim_sum: usize = 0;
    for _ in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: Usize1}
            if a[x] == 0 {
                more_then_ones.push(x);
            }
            // ^a[x] for cancellation.
            nim_sum ^= a[x] ^ (a[x] + 1);
            a[x] += 1;
        } else {
            for &x in more_then_ones.iter() {
                nim_sum ^= (a[x] - 1) ^ a[x];
                a[x] -= 1;
            }
            more_then_ones.retain(|&x| a[x] > 0);
        }
        println!("{}", nim_sum);
    }
}
