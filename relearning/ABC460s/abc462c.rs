use proconio::input;
// fix one axis
fn main() {
    input! {n: usize, mut xy: [(i64, i64); n]}
    xy.sort();
    let mut min_y: i64 = i64::MAX;
    let mut count: usize = 0;
    for i in 0..n {
        if min_y > xy[i].1 {
            count += 1;
            min_y = xy[i].1;
        }
    }
    println!("{}", count);
}
