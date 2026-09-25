use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {n: usize, a: [usize; n]}
    let mut heap = BinaryHeap::new();
    heap.push(a[0]);
    heap.push(a[1]);
    for i in 2..n {
        heap.push(a[i]);
        let first: usize = heap.pop().unwrap();
        let second: usize = heap.pop().unwrap();
        let third: usize = heap.pop().unwrap();
        println!("{}", third);
        heap.push(first);
        heap.push(second);
        heap.push(third);
    }
}
