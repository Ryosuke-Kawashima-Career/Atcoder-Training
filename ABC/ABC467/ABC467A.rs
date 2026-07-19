use proconio::input;

fn main() {
    input! {h: usize, w: usize}
    let index: usize = 25 * h * h;
    let weight: usize = w * 100 * 100;
    if weight >= index {
        println!("Yes");
    } else {
        println!("No");
    }
}
