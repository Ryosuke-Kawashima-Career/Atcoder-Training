use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    let mut sums: Vec<usize> = Vec::new();
    for (&key, &val) in map.iter() {
        sums.push(key * val);
    }
    sums.sort();
    let add_times: usize = sums.len().saturating_sub(k);
    let mut ans: usize = 0;
    for i in 0..add_times.min(sums.len()) {
        ans += sums[i];
    }
    println!("{}", ans);
}
