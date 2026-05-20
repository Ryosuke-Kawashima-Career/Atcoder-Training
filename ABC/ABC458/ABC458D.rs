use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
// ABC458D
// Q. Get the median of the numbers inserted so far.
// A. Two Binary Heaps
// low: maxheap for the smaller half
// high: minheap for the larger half
// invariant: low.len() >= high.len() and low.len() <= high.len() + 1
// invariant: all elements in low <= all elements in high
fn main() {
    input! {x: i64, q: usize}

    let mut low = BinaryHeap::new();
    let mut high = BinaryHeap::new();

    low.push(x);

    for _ in 0..q {
        input! {a: i64, b: i64}

        for &val in &[a, b] {
            if val <= *low.peek().unwrap() {
                low.push(val);
            } else {
                high.push(Reverse(val));
            }
        }

        while low.len() > high.len() + 1 {
            if let Some(top) = low.pop() {
                high.push(Reverse(top));
            }
        }
        while high.len() >= low.len() {
            if let Some(Reverse(top)) = high.pop() {
                low.push(top);
            }
        }

        println!("{}", low.peek().unwrap());
    }
}
