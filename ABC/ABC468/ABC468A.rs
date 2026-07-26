use proconio::input;

fn main() {
    input! {n: usize, a: [i64; n]}
    let mut ans: usize = 0;
    for i in 1..n - 1 {
        if a[i - 1] < a[i] && a[i] > a[i + 1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
