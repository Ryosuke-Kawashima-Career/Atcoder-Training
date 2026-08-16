use proconio::input;
use std::collections::BTreeSet;
fn main() {
    input!{n: usize, mut a: [i64; n]}
    a.sort();
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert(a[i]);
    }
    let mut cur_pos: i64 = 0;
    let mut cost: i64 = 0;
    for _ in 0..n {
        if let Some(left) = set.clone().range(..cur_pos).next_back() {
            if let Some(right) = set.clone().range(cur_pos..).next() {
                if (cur_pos - *left) <= (*right - cur_pos) {
                    cost += cur_pos - *left;
                    cur_pos = *left;
                    set.remove(left);
                } else {
                    cost += *right - cur_pos;
                    cur_pos = *right;
                    set.remove(right);
                }
            } else {
                cost += cur_pos - *left;
                cur_pos = *left;
                set.remove(left);
            }
        } else {
            if let Some(right) = set.clone().range(cur_pos..).next() {
                cost += *right - cur_pos;
                cur_pos = *right;
                set.remove(right);
            } else {
                break;
            }
        }
    }
    println!("{}", cost);
}
