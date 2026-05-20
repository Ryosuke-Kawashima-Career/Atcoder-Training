use proconio::input;
use std::collections::HashSet;
// abc403d
// Q. Any difference of a[i] and a[j] should not be d.
// Example. a = [1, 1, 3, 4, 5], d = 2 | Answer. 1
fn main() {
    input! {n: usize, d: i64, mut a: [i64; n]}
    a.sort();
    let (upper_bound1, to_be_erased1) = get_upper_bound(0, &a, d);
    let upper_bound2 = if to_be_erased1.len() == 0 {
        n
    } else {
        let (upper_bound2, _) = get_upper_bound(to_be_erased1[0], &a, d);
        upper_bound2
    };
    let upper_bound = upper_bound1.min(upper_bound2);
    println!("{}", upper_bound);
}

fn get_upper_bound(start: usize, a: &Vec<i64>, d: i64) -> (usize, Vec<usize>) {
    let n: usize = a.len();
    let mut a_no_dup = a.clone();
    a_no_dup.dedup();
    if start >= a_no_dup.len() {
        return (n, Vec::new());
    }
    let mut set = HashSet::new();
    set.insert(a_no_dup[start]);
    let mut cnt_erased: usize = 0;
    let mut to_be_erased: Vec<usize> = Vec::new();
    for i in 0..n {
        if set.contains(&(a[i] - d)) || set.contains(&(a[i] + d)) {
            cnt_erased += 1;
            to_be_erased.push(i);
        } else {
            set.insert(a[i]);
        }
    }
    return (cnt_erased, to_be_erased);
}
