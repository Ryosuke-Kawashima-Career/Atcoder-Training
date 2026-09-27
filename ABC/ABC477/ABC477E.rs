use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, q: usize, a: [i64; n], b: [i64; n]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n + 1];
    for i in 0..n {
        graph[i].push(((i + 1) % n, a[i]));
        graph[(i + 1) % n].push((i, a[i]));
        graph[i].push((n, b[i]));
        graph[n].push((i, b[i]));
    }
    for _query in 0..q {
        input! {s: Usize1, t: Usize1}
    }
}
