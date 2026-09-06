use proconio::input;

fn main() {
    input! {x: usize}
    for num in 1..=3 {
        if num != x {
            println!("{num}");
            break;
        }
    }
}
