use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, m: usize, s: Usize1, t: Usize1, mut ab: [(Usize1, Usize1); m]}
    ab.sort_by_key(|x| (x.1, x.0));
    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for &(v1, v2) in ab.iter() {
        if graph[v1].len() == 0 {
            graph[v1].push((v2, 0));
        } else {
            graph[v1].push((v2, 1));
        }
    }

    let dist: Vec<usize> = bfs01(&graph, s);
    if dist[t] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[t]);
    }
}

fn bfs01(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    let n = graph.len();
    let inf = usize::MAX;
    let mut dist: Vec<usize> = vec![inf; n];
    let mut deque = std::collections::VecDeque::new();
    dist[s] = 0;
    deque.push_back(s);
    while let Some(v) = deque.pop_front() {
        for &(u, w) in graph[v].iter() {
            if dist[u] > dist[v] + w {
                dist[u] = dist[v] + w;
                if w == 0 {
                    deque.push_front(u);
                } else {
                    deque.push_back(u);
                }
            }
        }
    }
    dist
}
