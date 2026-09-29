use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, m: usize, l: usize, s: i64, t: i64, uvc: [(Usize1, Usize1, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(u, v, w) in &uvc {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    let mut dist_from_start: Vec<Vec<Vec<i64>>> = vec![vec![vec![]; l + 1]; n];
    dist_from_start[0][0].push(0);
    dfs(0, 0, t, &graph, &mut dist_from_start);
    for v in 0..n {
        let mut is_ok: bool = false;
        for &distance in dist_from_start[v][l].iter() {
            if s <= distance && distance <= t {
                is_ok = true;
                break;
            }
        }
        if is_ok {
            print!("{} ", v + 1);
        }
    }
    print!("");
}

fn dfs(
    start: usize,
    phase: usize,
    t: i64,
    graph: &Vec<Vec<(usize, i64)>>,
    dist_from_start: &mut Vec<Vec<Vec<i64>>>,
) {
    let l: usize = dist_from_start[0].len() - 1;
    if phase == l {
        return;
    }
    for &dist in dist_from_start[start][phase].clone().iter() {
        for &(v, w) in graph[start].iter() {
            if dist + w > t {
                continue;
            }
            dist_from_start[v][phase + 1].push(dist + w);
        }
    }
}
