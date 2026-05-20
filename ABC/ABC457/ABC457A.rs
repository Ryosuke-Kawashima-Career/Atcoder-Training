use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n], x: usize}
    let ans = a[x - 1];
    println!("{}", ans);
}
