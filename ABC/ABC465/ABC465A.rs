use proconio::input;

fn main() {
    input! {a: usize, b: usize}
    if 3 * a > 2 * b {
        println!("Yes");
    } else {
        println!("No");
    }
}
