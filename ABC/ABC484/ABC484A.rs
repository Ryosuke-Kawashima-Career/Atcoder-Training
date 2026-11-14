use proconio::input;

fn main() {
    input! {b: usize, r: usize}
    let ans: usize = (30-b) * (30-r);
    println!("{}", ans);
}
