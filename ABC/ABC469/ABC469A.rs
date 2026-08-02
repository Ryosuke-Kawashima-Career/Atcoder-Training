use proconio::input;

fn main() {
    input! {n: usize, k: usize}
    let ans = n + 1 - k;
    println!("{}", ans);
}
