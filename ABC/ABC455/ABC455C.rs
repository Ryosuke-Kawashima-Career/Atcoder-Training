use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}
    let mut count_a: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..n {
        count_a.insert(a[i], count_a.get(&a[i]).unwrap_or(&0) + 1);
    }
    let mut bt: BTreeMap<usize, usize> = BTreeMap::new();
    let mut remaining: usize = count_a.len().saturating_sub(k);
    for (&val, &num) in count_a.iter() {
        let entry = bt.entry(val * num).or_insert(0);
        *entry += num;
    }
    let mut ans: usize = 0;
    for (&val, &product_count) in bt.iter() {
        if remaining == 0 {
            break;
        }
        remaining -= 1;
        ans += val;
    }
    println!("{}", ans);
}
