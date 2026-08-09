use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, c: [Usize1; n]}
    let mut max_count: usize = 0;
    let mut count: Vec<usize> = vec![0; n];
    for i in 0..n {
        count[c[i]] += 1;
        max_count = max_count.max(count[c[i]]);
    }
    let ans: usize = n - max_count;
    println!("{}", ans);
}
