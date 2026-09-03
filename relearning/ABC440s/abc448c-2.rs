use proconio::input;
use proconio::marker::Usize1;
// ABC448C
// Q. Find the value after the values of B are removed from A.
// A. Binary Search
// X being minimum means that there is no value less than X => lower_bound(X) == 0
fn main() {
    input! {n: usize, q: usize, a: [i64; n]}
    let mut a_sorted: Vec<i64> = a.clone();
    a_sorted.sort();
    for _case in 0..q {
        input! {k: usize, b: [Usize1; k]}
        let mut b_sorted: Vec<i64> = Vec::new();
        for &i in b.iter() {
            b_sorted.push(a[i]);
        }
        b_sorted.sort();
        let ans = binary_search(&a_sorted, &b_sorted);
        println!("{}", ans);
    }
}

fn binary_search(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    let mut left: i64 = 0;
    let mut right: i64 = 1_000_000_000;
    while right - left > 1 {
        let mid: i64 = (left + right) / 2;
        let a_less_than_mid: usize = a.partition_point(|&x| x < mid);
        let b_less_than_mid: usize = b.partition_point(|&x| x < mid);
        // Binary Search of the answer
        if a_less_than_mid == b_less_than_mid {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}
