use proconio::input;

fn main() {
    input! {n: usize, a: [i64; n]}
    let upper_half: usize = n / 2;
    let mut ans: i64 = 0;
    for i in upper_half..n {
        ans += a[i];
    }
    println!("{}", ans);
}
