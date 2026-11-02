use proconio::input;

fn main() {
    input! {n: usize, mut x: [usize; n]}
    x.sort();
    let x_min: usize = x[0];
    let x_max: usize = x[n - 1];
    for k in 0..n {
        let ans = ((x[k] - x_min) / 2).max((x_max - x[k]) / 2);
        println!("{}", ans);
    }
}
