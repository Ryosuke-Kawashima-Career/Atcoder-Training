use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {q: usize, queries: [(usize, usize); q]}
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut answer: Vec<usize> = vec![0; q];
    let mut cur_trees: usize = 0;
    for (i, &(query_type, h)) in queries.iter().enumerate() {
        if query_type == 1 {
            cur_trees += 1;
            *map.entry(h).or_insert(0) += 1;
            answer[i] = cur_trees;
        } else {
            cur_trees = 0;
            answer[i] = cur_trees;
        }
    }

    for i in (0..q).rev() {
        let (query_type, h) = queries[i];
        if query_type == 2 {}
    }
}
