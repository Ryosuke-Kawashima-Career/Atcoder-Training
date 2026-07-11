use proconio::input;

fn main() {
    input! {n: usize, x: [i64; n]}
    let mut all_negative: bool = true;
    for i in 0..n {
        if x[i] > 0 {
            all_negative = false;
            break;
        }
    }
    if all_negative {
        println!("Yes");
    } else {
        println!("No");
    }
}
