use proconio::{input, marker::Usize1};

fn main() {
    input!{n: usize, uv: [(Usize1, Usize1); n-1], c: [Usize1; n]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    let dist_from_0 = bfs(&graph, 0);
    let max_dist: usize = *dist_from_0.iter().max().unwrap();
    let mut max_dist_nodes: Vec<usize> = vec![];
    for i in 0..n {
        if dist_from_0[i] == max_dist {
            max_dist_nodes.push(i);
        }
    }
    let mut ans: usize = 0;
    for &max_dist_node in &max_dist_nodes {
        let dist_from_max_dist_node = bfs(&graph, max_dist_node);
        let diameter = *dist_from_max_dist_node.iter().max().unwrap();
        for v in 0..n {
            if dist_from_max_dist_node[v] == diameter {
                if c[max_dist_node] == c[v] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let n: usize = graph.len();
    let mut dist = vec![std::usize::MAX; graph.len()];
    dist[start] = 0;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if dist[v] == std::usize::MAX {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    return dist;
}
