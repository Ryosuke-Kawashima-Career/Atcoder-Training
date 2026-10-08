use proconio::input;
fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n], c: [usize; n]}
    let mut prefix_a: Vec<usize> = vec![0; n + 1];
    let mut prefix_b: Vec<usize> = vec![0; n + 1];
    let mut prefix_c: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        prefix_a[i + 1] = prefix_a[i] + a[i];
        prefix_b[i + 1] = prefix_b[i] + b[i];
        prefix_c[i + 1] = prefix_c[i] + c[i];
    }
    let mut ans: usize = 0;
    let mut best_of_x: usize = 0;
    for y in 2..n {
        let x: usize = y - 1;
        let candidate_x: usize = prefix_a[x] + prefix_b[x];
        best_of_x = best_of_x.max(candidate_x);
        let score: usize = best_of_x + prefix_b[n] - prefix_b[y] + prefix_c[n];
        ans = ans.max(score);
    }
    println!("{}", ans);
}
