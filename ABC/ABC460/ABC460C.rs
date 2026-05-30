use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input!{n: usize, m: usize, mut a: [usize; n], mut b: [usize; m]}
    a.sort();
    b.sort_by(|x1, x2| x2.cmp(&x1));

    let mut ans: usize = 0;
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for neta in 0..m {
        *map.entry(b[neta]).or_insert(0) += 1;
    }

    for shari in 0..n {
        // 1. Find the key first. We use `map(|(&k, _)| k)` to copy the key out 
        // and immediately release the immutable borrow on `map`.
        let target_key = map.range(..=2 * a[shari]).last().map(|(&k, _)| k);
        
        // 2. Mutate the map safely now that the previous borrow is gone.
        if let Some(key) = target_key {
            let val = map.get_mut(&key).unwrap();
            *val -= 1;
            if *val == 0 {
                map.remove(&key);
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
