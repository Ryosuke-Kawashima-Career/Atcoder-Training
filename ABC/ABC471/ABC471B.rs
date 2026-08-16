use proconio::input;
use std::collections::HashMap;
fn main() {
    input!{n: usize, s: [String; n]};
    let mut map = HashMap::new();
    let mut ans: usize = 0;
    for i in 0..n {
        let t: String = s[i].to_lowercase();
        *map.entry(t.clone()).or_insert(0) += 1;
        let cur: usize = map[&t];
        ans = ans.max(cur);
    }
    println!("{}", ans);
}
