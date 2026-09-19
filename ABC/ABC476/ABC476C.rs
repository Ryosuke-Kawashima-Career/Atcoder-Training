use proconio::input;
use std::collections::BiaryHeap;
fn main() {
    input! {n: usize, a: [usize; n]}
    let mut bh = BiaryHeap::new();
    bh.push(a[0]);
    bh.push(a[1]);
    let mut answer: Vec<usize> = Vec::new();
    for k in 2..n {
        bh.push(a[k]);
        let first = bh.pop();
        let second = bh.pop();
        let third = bh.pop();
        answer.push(third);
        bh.push(first);
        bh.push(second);
        bh.push(third);
    }
    for i in 0..n - 2 {
        print!("{} ", answer[i]);
    }
    println!("");
}
