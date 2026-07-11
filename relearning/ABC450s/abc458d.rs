use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! { x: i64, q: usize, ab: [(i64, i64); q] }
    // smaller half (max-heap): top = current median candidate
    let mut lower: BinaryHeap<i64> = BinaryHeap::new();
    // larger half (min-heap via Reverse)
    let mut upper: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut median: i64 = x;
    lower.push(x);

    for &(num1, num2) in ab.iter() {
        for num in [num1, num2] {
            if num <= median {
                lower.push(num);
            } else {
                upper.push(Reverse(num));
            }
        }

        // lower can now have up to 2 extra elements -> push the excess to upper
        while lower.len() > upper.len() + 1 {
            if let Some(val) = lower.pop() {
                upper.push(Reverse(val));
            }
        }
        // upper can now have up to 1 extra element -> pull it back into lower
        while lower.len() <= upper.len() {
            if let Some(Reverse(val)) = upper.pop() {
                lower.push(val);
            }
        }

        let cur_median: i64 = *lower.peek().unwrap();
        median = cur_median;
        println!("{}", cur_median);
    }
}