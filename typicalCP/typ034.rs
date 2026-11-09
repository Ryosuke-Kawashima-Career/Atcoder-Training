use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut count: HashMap<usize, usize> = HashMap::new();
    let mut ans: usize = 0;
    let mut right: usize = 0;

    for left in 0..n {
        while right < n && (count.len() < k || count.contains_key(&a[right])) {
            *count.entry(a[right]).or_insert(0) += 1;
            right += 1;
        }

        ans = ans.max(right - left);

        if let Some(val) = count.get_mut(&a[left]) {
            if *val == 1 {
                count.remove(&a[left]);
            } else {
                *val -= 1;
            }
        }
    }

    println!("{}", ans);
}
