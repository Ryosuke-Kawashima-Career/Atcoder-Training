use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, m: usize, edges: [(Usize1, Usize1); m]}
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for &(a, b) in edges.iter() {
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut colors: Vec<isize> = vec![-1; n];
        let mut parent: Vec<usize> = (0..n).collect();
        let (is_bipartite, odd_cycle_edge) = judge_bipartite(&graph, &mut colors, &mut parent);
        if !is_bipartite && odd_cycle_edge.is_some() {
            let path: Vec<usize> = get_path(&parent, odd_cycle_edge);
            let k: usize = path.len();
            println!("{}", k);
            for i in 0..k {
                print!("{}{}", path[i] + 1, if i + 1 == k { "" } else { " " });
            }
            println!("");
        } else {
            println!("-1");
        }
    }
}

fn judge_bipartite(
    graph: &Vec<Vec<usize>>,
    colors: &mut Vec<isize>,
    parent: &mut Vec<usize>,
) -> (bool, Option<(usize, usize)>) {
    let n: usize = graph.len();
    let mut que = VecDeque::new();
    let mut dist: Vec<usize> = vec![usize::MAX; n];

    for v in 0..n {
        if colors[v] == -1 {
            colors[v] = 0;
            dist[v] = 0;
            que.push_back(v);

            while let Some(cur_v) = que.pop_front() {
                for &next in graph[cur_v].iter() {
                    if dist[next] == usize::MAX {
                        dist[next] = dist[cur_v] + 1;
                        colors[next] = 1 - colors[cur_v];
                        parent[next] = cur_v;
                        que.push_back(next);
                    } else if dist[cur_v] == dist[next] {
                        // In BFS, an edge between two vertices at the same depth indicates an odd cycle.
                        return (false, Some((cur_v, next)));
                    }
                }
            }
        }
    }
    (true, None)
}

fn get_path(parent: &Vec<usize>, odd_cycle_edge: Option<(usize, usize)>) -> Vec<usize> {
    let (u, v) = odd_cycle_edge.unwrap();
    let mut cur_u: usize = u;
    let mut cur_v: usize = v;
    let mut path_u: Vec<usize> = vec![u];
    let mut path_v: Vec<usize> = vec![v];

    while cur_u != cur_v {
        cur_u = parent[cur_u];
        cur_v = parent[cur_v];
        path_u.push(cur_u);
        path_v.push(cur_v);
    }

    path_v.pop();
    path_v.reverse();
    path_u.extend(path_v);
    path_u
}
