use proconio::input;

fn main() {
    input! {h: usize, w: usize, n: usize, q: usize, rc: [(usize, usize); n], queries: [(usize, usize, usize); q]}
    let mut rows: Vec<usize> = Vec::new();
    let mut cols: Vec<usize> = Vec::new();
    for &(row, col) in rc.iter() {
        rows.push(row);
        cols.push(col);
    }
    rows.sort();
    cols.sort();
    for query in queries.iter() {
        if query.0 == 1 {
            let (u, d) = (query.1, query.2);
            let up_count = rows.partition_point(|&x| x < u);
            let down_count = rows.partition_point(|&x| x <= d);
            let ans = down_count - up_count;
            println!("{}", ans);
        } else {
            let (l, r) = (query.1, query.2);
            let left_count = cols.partition_point(|&x| x < l);
            let right_count = cols.partition_point(|&x| x <= r);
            let ans = right_count - left_count;
            println!("{}", ans);
        }
    }
}