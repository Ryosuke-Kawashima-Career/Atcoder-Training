use proconio::input;
const INF: i64 = 1 << 60;
fn main() {
    input!{n: usize, a: [i64; n]}
    let mut positions: Vec<i64> = vec![0; n+1];
    for i in 0..n {
        positions[i+1] = positions[i] + a[i];
    }
    let mut prefix_minima: Vec<i64> = vec![INF; n+1];
    let mut prefix_maxima: Vec<i64> = vec![-INF; n+1];
    let mut suffix_minima: Vec<i64> = vec![INF; n+1];
    let mut suffix_maxima: Vec<i64> = vec![-INF; n+1];
    prefix_maxima[0] = 0;
    prefix_minima[0] = 0;
    for i in 0..n {
        prefix_maxima[i+1] = prefix_maxima[i].max(positions[i+1]);
        prefix_minima[i+1] = prefix_minima[i].min(positions[i+1]);
    }
    for i in (0..n).rev() {
        suffix_maxima[i] = suffix_maxima[i+1].max(positions[i]);
        suffix_minima[i] = suffix_minima[i+1].min(positions[i]);
    }
    let mut ans: i64 = 0;
    for i in 0..n {
        let prefix_minimum: i64 = prefix_minima[i+1];
        let suffix_maximum: i64 = suffix_maxima[i+1] - 2 * a[i];
        ans = ans.max(suffix_maximum - prefix_minimum + 1);
        let prefix_maximum: i64 = prefix_maxima[i+1];
        let suffix_minimum: i64 = suffix_minima[i+1] - 2 * a[i];
        ans = ans.max(prefix_maximum - suffix_minimum + 1);
    }
    println!("{}", ans);
}
