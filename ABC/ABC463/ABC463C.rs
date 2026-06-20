use proconio::input;
use std::collections::BinaryHeap;
fn main() {
    input! {n: usize, hl: [(usize, usize); n], q: usize, times: [usize; q]}
    // (time, query_type, index)
    let mut queries: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..n {
        queries.push((hl[i].1, 0, i));
    }
    for i in 0..q {
        queries.push((times[i], 1, i));
    }
    queries.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            b.0.cmp(&a.0)
        }
    });
    let mut answers: Vec<usize> = vec![0; q];
    let mut bh = BinaryHeap::new();
    let mut cur_time: usize = queries[queries.len() - 1].0;
    let mut cur_index: usize = queries.len() - 1;
    while cur_index > 0 {
        let query_type: usize = queries[cur_index].1;
        if query_type == 0 {
            bh.push(hl[queries[cur_index].2].0);
        } else {
            answers[queries[cur_index].2] = *bh.peek().unwrap();
        }
        cur_index -= 1;
    }
    for i in 0..q {
        println!("{}", answers[i]);
    }
}
