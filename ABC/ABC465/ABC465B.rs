use proconio::input;

fn main() {
    input! {x: usize, y: usize, l: usize, r: usize, a: usize,  b: usize}
    let mut ans: usize = 0;
    for hour in 1..24 {
        if l <= hour && hour < r {
            if a <= hour && hour < b {
                ans += x;
            }
        } else {
            if a <= hour && hour < b {
                ans += y;
            }
        }
    }
    println!("{}", ans);
}
