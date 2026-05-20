use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let max_a: usize = *a.iter().max().unwrap();
    let mut b: Vec<usize> = Vec::new();
    for i in 0..n {
        let diff: usize = max_a - a[i];
        let count: usize = diff / k;
        let element: usize = a[i] + k * count;
        b.push(element);
    }
    b.sort_unstable();
    let mut min_diff: usize = b[n - 1] - b[0];
    for i in 0..n - 1 {
        let next_max: usize = b[i] + k;
        let next_min: usize = b[i + 1];
        let next_diff: usize = next_max - next_min;
        min_diff = min_diff.min(next_diff);
    }
    println!("{}", min_diff);
}
