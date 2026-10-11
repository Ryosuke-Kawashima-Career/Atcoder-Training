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
        let ans: usize =
            (min_for_r[s - 1] + min_for_l[t - 1]).min(min_for_l[s - 1] + min_for_r[t - 1]);
        println!("{ans}")
    }
}
