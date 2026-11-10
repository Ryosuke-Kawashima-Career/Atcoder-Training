use proconio::input;
const INF: i64 = 1 << 60;
fn main() {
    input!{n: usize, q: usize, xy: [(i64, i64); n]}
    let mut manhattan_coord: Vec<(i64, i64)> = Vec::new();
    let mut y_min: i64 = INF;
    let mut y_max: i64 = -INF;
    let mut x_min: i64 = INF;
    let mut x_max: i64 = -INF;
    for &(x, y) in xy.iter() {
        let x_manhattan: i64 = x - y;
        let y_manhattan: i64 = x + y;
        manhattan_coord.push((x_manhattan, y_manhattan));
        x_min = x_min.min(x_manhattan);
        x_max = x_max.max(x_manhattan);
        y_min = y_min.min(y_manhattan);
        y_max = y_max.min(y_manhattan);
    }
    for point in 0..n {
        let candidiate1: i64 = (manhattan_coord[point].0-manhattan_coord[0].0).abs();
        let candidiate2: i64 = (manhattan_coord[point].1-manhattan_coord[0].1).abs();
        let ans: i64 = candidiate1.max(candidiate2);
        println!("{}", ans);
    }
}
