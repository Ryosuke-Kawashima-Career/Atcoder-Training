use proconio::input;

fn main() {
    input! {x: usize, y: usize}
    if x * 9 == y * 16 {
        println!("Yes");
    } else {
        println!("No");
    }
}
