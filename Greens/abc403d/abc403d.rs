use proconio::input;
use std::collections::HashSet;
// abc403d
// Q. Any difference of a[i] and a[j] should not be d.
// Example. a = [1, 1, 3, 4, 5], d = 2 | Answer. 1
fn main() {
    input! {n: usize, d: i64, mut a: [i64; n]}
    a.sort();
    let upper_bound1 = get_upper_bound(0, &a, d);
    let upper_bound2 = get_upper_bound(1, &a, d);
    let upper_bound = upper_bound1.min(upper_bound2);
    println!("{}", upper_bound);
}

fn get_upper_bound(start: usize, a: &Vec<i64>, d: i64) -> usize {
    let n: usize = a.len();
    let mut a_no_dup = a.clone();
    a_no_dup.dedup();
    if start >= a_no_dup.len() {
        return n;
    }
    let mut set = HashSet::new();
    set.insert(a_no_dup[start]);
    let mut to_be_erased: usize = 0;
    for i in 0..n {
        if set.contains(&(a[i] - d)) || set.contains(&(a[i] + d)) {
            to_be_erased += 1;
        } else {
            set.insert(a[i]);
        }
    }
    return to_be_erased;
}
