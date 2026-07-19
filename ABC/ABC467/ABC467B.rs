use proconio::input;

fn main() {
    input! {n: usize, shoppings: [(i64, i64, String); n]}
    let mut x: i64 = 10000;
    let mut y: i64 = 10000;
    for i in 0..n {
        let (a, b, s) = &shoppings[i];
        y -= a;
        if s == "keep" {
            x -= b;
        } else {
            x -= a;
        }
    }
    let ans = y - x;
    println!("{}", ans);
}
