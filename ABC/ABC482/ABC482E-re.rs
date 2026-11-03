use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
        color: [Usize1; n],
    }
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    let dist_from_0 = bfs(&graph, 0);
    let max_dist: usize = *dist_from_0.iter().max().unwrap();
    let mut max_dist_node: usize = 0;
    for i in 0..n {
        if dist_from_0[i] == max_dist {
            max_dist_node = i;
            break;
        }
    }
    let dist_from_max_dist_node = bfs(&graph, max_dist_node);
    let diameter: usize = *dist_from_max_dist_node.iter().max().unwrap();
    let mut end_node: usize = 0;
    for i in 0..n {
        if dist_from_max_dist_node[i] == diameter {
            end_node = i;
            break;
        }
    }
    let target_depth: usize = diameter / 2;
    let mut ans: usize = 0;
    if diameter % 2 == 0 {
        let center: usize = find_center_even(&graph, end_node, &dist_from_max_dist_node);
        let mut count_total: Vec<usize> = vec![0; n];
        for &u in &graph[center] {
            let mut subtree_colors: Vec<usize> = Vec::new();
            dfs_even(&graph, u, center, 1, target_depth, &mut subtree_colors, &color);
            for &c in &subtree_colors {
                ans += count_total[c];
            }
            for &c in &subtree_colors {
                count_total[c] += 1;
            }
        }
    } else {
        let (center1, center2): (usize, usize) =
            find_center_odd(&graph, end_node, &dist_from_max_dist_node);
        let mut count_total1: Vec<usize> = vec![0; n];
        let mut count_total2: Vec<usize> = vec![0; n];
        dfs_odd(&graph, center1, center2, 0, target_depth, &mut count_total1, &color);
        dfs_odd(&graph, center2, center1, 0, target_depth, &mut count_total2, &color);
        for c in 0..n {
            ans += count_total1[c] * count_total2[c];
        }
    }
    println!("{}", ans);
}

fn find_center_even(graph: &Vec<Vec<usize>>, start: usize, dist: &Vec<usize>) -> usize {
    let mut current: usize = start;
    let target_dist: usize = dist[start] / 2;
    while dist[current] != target_dist {
        for &next in &graph[current] {
            if dist[next] < dist[current] {
                current = next;
                break;
            }
        }
    }
    return current;
}

fn find_center_odd(graph: &Vec<Vec<usize>>, start: usize, dist: &Vec<usize>) -> (usize, usize) {
    let mut current1: usize = start;
    let target_dist: usize = dist[start] / 2;
    while dist[current1] != target_dist {
        for &next in &graph[current1] {
            if dist[next] < dist[current1] {
                current1 = next;
                break;
            }
        }
    }
    let mut current2: usize = start;
    let target_dist2: usize = dist[start] / 2 + 1;
    while dist[current2] != target_dist2 {
        for &next in &graph[current2] {
            if dist[next] < dist[current2] {
                current2 = next;
                break;
            }
        }
    }
    return (current1, current2);
}

fn dfs_even(
    graph: &Vec<Vec<usize>>,
    v: usize,
    parent: usize,
    depth: usize,
    target_depth: usize,
    subtree_colors: &mut Vec<usize>,
    color: &Vec<usize>,
) {
    if depth == target_depth {
        subtree_colors.push(color[v]);
        return;
    }
    for &u in &graph[v] {
        if u != parent {
            dfs_even(graph, u, v, depth + 1, target_depth, subtree_colors, color);
        }
    }
}

fn dfs_odd(
    graph: &Vec<Vec<usize>>,
    v: usize,
    parent: usize,
    depth: usize,
    target_depth: usize,
    count_total: &mut Vec<usize>,
    color: &Vec<usize>,
) {
    if depth == target_depth {
        count_total[color[v]] += 1;
        return;
    }
    for &u in &graph[v] {
        if u != parent {
            dfs_odd(graph, u, v, depth + 1, target_depth, count_total, color);
        }
    }
}

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
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
