use proconio::input;

fn main() {
    input!{a: i64, b: i64}
    let add: i64 = a + b;
    let sub: i64 = a - b;
    let mul: i64 = a * b;
    if add == 9 || sub == 9 || mul == 9 || a == 9 * b {
        println!("Nine");
    } else {
        pirnt!("Nein");
    }
}
