use proconio::input;
const INF: usize = 1 << 60;
// Binary search => Focus on the monotonicity
fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let mut min_left: usize = 0;
    let mut min_right: usize = INF;
    while min_right - min_left > 1 {
        let min_med: usize = (min_left + min_right) / 2;
        if count_operations(min_med, &a) <= k {
            min_left = min_med;
        } else {
            min_right = min_med;
        }
    }
    println!("{}", min_left);
}

fn count_operations(min_value: usize, array: &Vec<usize>) -> usize {
    /*Judge function for the binary search
     */
    let mut operations: usize = 0;
    let n: usize = array.len();
    for i in 1..=n {
        let diff: usize = if array[i - 1] < min_value {
            min_value - array[i - 1]
        } else {
            0
        };
        if diff > 0 {
            operations += 1.max(diff / i);
        }
    }
    return operations;
}
