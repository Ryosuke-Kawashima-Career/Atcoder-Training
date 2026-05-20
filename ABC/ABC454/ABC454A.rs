use proconio::input;

fn main() {
    input! {l: usize, r: usize}
    let ans = r - l + 1;
    println!("{}", ans);
}
