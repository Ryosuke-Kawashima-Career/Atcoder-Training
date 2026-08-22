use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn solve() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dist = vec![None; n];
    let mut parent = vec![None; n];
    let mut que = VecDeque::new();

    dist[0] = Some(0);
    que.push_back(0);

    let mut odd_cycle_edge: Option<(usize, usize)> = None;

    'bfs: while let Some(u) = que.pop_front() {
        let d_u = dist[u].unwrap();
        for &v in &graph[u] {
            if let Some(d_v) = dist[v] {
                if d_u == d_v {
                    odd_cycle_edge = Some((u, v));
                    break 'bfs;
                }
            } else {
                dist[v] = Some(d_u + 1);
                parent[v] = Some(u);
                que.push_back(v);
            }
        }
    }

    if let Some((u, v)) = odd_cycle_edge {
        let mut path_u = vec![u];
        let mut path_v = vec![v];
        let mut curr_u = u;
        let mut curr_v = v;

        while curr_u != curr_v {
            curr_u = parent[curr_u].unwrap();
            curr_v = parent[curr_v].unwrap();
            path_u.push(curr_u);
            path_v.push(curr_v);
        }

        path_v.pop();
        path_v.reverse();
        path_u.extend(path_v);

        let k = path_u.len();
        println!("{}", k);
        for i in 0..k {
            print!("{}{}", path_u[i] + 1, if i + 1 == k { "" } else { " " });
        }
        println!();
    } else {
        println!("-1");
    }
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}
