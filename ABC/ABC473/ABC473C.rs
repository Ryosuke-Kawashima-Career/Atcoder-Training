use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;
fn main() {
    input! {n: usize, k: usize, a: [Usize1; n]}
    let mut class_members: Vec<usize> = vec![0; k];
    for i in 0..n {
        class_members[a[i]] += 1;
    }
    let mut map = BTreeMap::new();
    for class in 0..k {
        *map.entry(class_members[class]).or_insert(0) += 1;
    }
    let mut ans: usize = 0;
    let (larget_number, larget_count) = map.iter().next_back().unwrap();
    ans += *larget_count;
    if let Some((second_larget, second_count)) = map.iter().rev().nth(1) {
        if *second_larget + 1 == *larget_number {
            ans += *second_count;
        }
    }
    println!("{}", ans);
}
