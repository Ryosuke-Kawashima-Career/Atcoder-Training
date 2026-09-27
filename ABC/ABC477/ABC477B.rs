use proconio::input;

fn main() {
    input! {n: usize, d: i64, x: [i64; n]}
    let mut coord: Vec<(i64, usize)> = Vec::new();
    for i in 0..n {
        coord.push((x[i], i));
    }
    coord.sort();
    let mut distincts: Vec<usize> = Vec::new();
    for i in 0..n {
        let mut before_x: i64 = i64::MIN;
        let mut after_x: i64 = i64::MAX;
        if i > 0 {
            before_x = coord[i - 1].0;
        }
        if i < n - 1 {
            after_x = coord[i + 1].0;
        }
        let mut is_distinct: bool = true;
        if before_x != i64::MIN && before_x + d > coord[i].0 {
            is_distinct = false;
        }
        if after_x != i64::MAX && after_x - d < coord[i].0 {
            is_distinct = false;
        }
        if is_distinct {
            distincts.push(coord[i].1);
        }
    }
    println!("{}", distincts.len());
    for i in 0..distincts.len() {
        print!("{} ", distincts[i] + 1);
    }
    println!("");
}
