use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;
// use std::cmp::Reverse;
fn main() {
    input! {n: usize, q: usize, a: [i64; n]}
    let mut map = BTreeMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    for _case in 0..q {
        input! {k: usize, b: [Usize1; k]}
        for i in 0..k {
            if let Some(count) = map.get_mut(&a[b[i]]) {
                *count -= 1;
                if *count == 0 {
                    map.remove(&a[b[i]]);
                }
            }
        }
        let ans: i64 = *map.iter().next().unwrap().0;
        println!("{ans}");
        for i in 0..k {
            *map.entry(a[b[i]]).or_insert(0) += 1;
        }
    }
}
