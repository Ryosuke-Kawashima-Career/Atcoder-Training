use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {n: usize, m: usize, mut a: [usize; n], b: [usize; m]}
    a.sort();
    let mut map = BTreeMap::new();
    for &val in b.iter() {
        *map.entry(val).or_insert(0) += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        if let Some((&key, _)) = map.range(..=2 * a[i]).next_back() {
            ans += 1;
            *map.entry(key).or_insert(0) -= 1;
            if map.get(&key).unwrap() == &0 {
                map.remove(&key);
            }
        }
    }
    println!("{}", ans);
}
