use proconio::input;

fn main() {
    input! {x: usize}
    if 3 <= x && x <= 18 {
        println!("Yes");
    } else {
        println!("No");
    }
}
