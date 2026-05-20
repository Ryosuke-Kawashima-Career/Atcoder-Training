use proconio::{input, marker::Usize1};
use std::collections::HashMap;
// ABC 402 D
// Q. There are n points numbered 1 to n on a circle. There are m chords connecting pairs of these points.
// Find the number of pairs of chords that intersect inside the circle.
// A. Complementary counting. 余事象を数える。(交差の反対は平行)
// A. 平行な直線の組み合わせを数え、その組み合わせ数を全組み合わせ数から引く。(modが鍵)
fn main() {
    input!{n: usize, m: usize, ab: [(Usize1, Usize1); m]}
    let mut parallels: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for i in 0..m {
        let (a, b) = ab[i];
        // (0, end_parallel): Representation of parallel lines
        let end_parallel: usize = (a + b) % n;
        let key = (0, end_parallel);
        (parallels.entry(key).or_insert(vec![])).push(i);
    }
    let mut ans: usize = 0;
    for (_key, vec) in parallels.iter() {
        let len = vec.len();
        ans += len * (m - len);
    }
    // Erase double counting
    println!("{}", ans / 2);
}