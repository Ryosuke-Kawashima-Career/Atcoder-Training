use proconio::input;

fn main() {
    input! {n: usize}
    for num in (1..=n).rev() {
        if num == n {
            print!("{}", num);
        } else {
            print!(",{}", num);
        }
    }
    println!("");
}
