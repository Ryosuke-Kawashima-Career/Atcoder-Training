use proconio::input;

fn main() {
    input!{a: i64, b: i64}
    if a * b > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
