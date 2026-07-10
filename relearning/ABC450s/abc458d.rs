use proconio::input;
use std::collections::BinaryHeap;
ust std::cmp::Reverse;
fn main() {
    input!{x: i64, q: usize, ab: [(i64, i64); q]}
    // smaller ==> bigger
    let mut lower = BinaryHeap::new();
    // smaller <== bigger
    let mut upper = BinaryHeap::new();
    let mut median: i64 = x;
    lower.push(x);
    for &(num1, num2) in ab.iter() {
        if num1 <= median {
            lower.push(num1);
        } else {
            upper.push(Reverse(num1));
        }
        if num2 <= median {
            lower.push(num2);
        } else {
            upper.push(Reverse(num2));
        }
        while lower.len() <= upper.len() {
            if let Some(Reverse(val)) = upper.pop() {
                lower.push(val);
            }
        }
        let cur_median: i64 = lower.peek().unwrap();
        median = cur_median;
        println!("{}", cur_median);
    }
}
