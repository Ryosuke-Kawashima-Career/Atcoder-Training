use proconio::input_interactive;
fn main() {
    input_interactive! {n: usize}
    let mut points: Vec<i64> = vec![0; n];
    let mut ans: usize = 0;
    let mut left: usize = 1;
    for i in 0..n - 1 {
        let mut right = left;
        while query(i, right) {
            right += 1;
        }
        let count: usize = right - i;
        ans += count * (count - 1) / 2;
        left = right;
    }
    println!("! {}", ans);
}

fn query(i: usize, j: usize) -> bool {
    println!("? {} {}", i + 1, j + 1);
    input_interactive! {ans: String}
    return &ans == "Yes";
}
