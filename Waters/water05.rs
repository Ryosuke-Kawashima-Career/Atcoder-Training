use proconio::input;

fn main() {
    // A pizza, B pizza, AB pizza, x pieces, y pieces
    input! {a: usize, b: usize, c: usize, x: usize, y: usize}
    let mut ans: usize = 0;
    if a + b >= 2 * c {
        let min_num: usize = x.min(y);
        let max_num: usize = x.max(y);
        ans += min_num * 2 * c;
        let diff: usize = max_num - min_num;
        let a_or_b: usize = if max_num == x { a } else { b };
        if a_or_b >= 2 * c {
            ans += diff * 2 * c;
        } else {
            ans += diff * a_or_b;
        }
    } else {
        ans = x * a + y * b;
    }
    println!("{}", ans);
}
