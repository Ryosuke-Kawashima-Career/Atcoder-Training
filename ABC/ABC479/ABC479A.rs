use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    let mut ans: usize = 0;
    for i in (0..n).step_by(2) {
        ans += a[i];
    }
    println!("{}", ans);
}
