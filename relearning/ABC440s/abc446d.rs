use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {n: usize, a: [i64; n]}
    let mut last_elem_to_length: BTreeMap<i64, usize> = BTreeMap::new();
    for i in 0..n {
        if let Some(prev_length) = last_elem_to_length.get(&(a[i] - 1)) {
            let curr_length: usize = prev_length + 1;
            *last_elem_to_length.entry(a[i]).or_default() =
                std::cmp::max(*last_elem_to_length.get(&a[i]).unwrap_or(&0), curr_length);
        } else {
            *last_elem_to_length.entry(a[i]).or_default() =
                std::cmp::max(*last_elem_to_length.get(&a[i]).unwrap_or(&0), 1);
        }
    }
    let ans = *last_elem_to_length.values().max().unwrap_or(&0);
    println!("{}", ans);
}
