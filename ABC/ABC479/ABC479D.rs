use proconio::input;
fn main() {
    input! {n: usize, q: usize, l: [usize; n-1], r: [usize; n-1], queries: [(usize, usize); q]}
    let mut min_for_l: Vec<usize> = vec![usize::MAX; n];
    let mut min_for_r: Vec<usize> = vec![usize::MAX; n];
    for i in (0..n - 1).rev() {
        min_for_r[i] = min_for_r[i + 1].min(r[i]);
    }
    for i in 1..n {
        min_for_l[i] = min_for_l[i - 1].min(l[i - 1]);
    }
    for &(s, t) in queries.iter() {
        let (s, t) = (s - 1, t - 1);
        let ans: usize = if s < t {
            if s == 0 {
                r[s]
            } else {
                r[s].min(l[s - 1] + min_for_r[s])
            }
        } else {
            if s == n - 1 {
                l[s]
            } else {
                l[s].min(r[s + 1] + min_for_l[s])
            }
        };
        println!("{ans}");
    }
}
