use proconio::input;

fn main() {
    input! {l: i64, r: i64, d: i64, u: i64}
    let mut ans: i64 = 0;
    for y in d..=u {
        let curr: i64 = get_value(l, r, y);
        ans += curr;
    }
    println!("{}", ans);
}

fn get_value(l: i64, r: i64, y: i64) -> i64 {
    let x_left: i64 = -(y.abs());
    let x_right: i64 = y.abs();

    if x_left > r || l > x_right {
        return (r - l) / 2;
    }
    let mut result: i64 = 0;
    if l <= x_left {
        result += (x_left - l) / 2;
    }
    result += x_left.abs() / 2;
    if r >= x_right {
        result += (r - x_right) / 2;
    }
    result += x_right / 2;
    result
}
