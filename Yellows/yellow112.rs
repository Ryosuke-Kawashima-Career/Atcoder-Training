use proconio::{input, marker::Usize1};
// abc014d
// Q. what is the minimum size of the cyclic path between node1 and node2 when adding a new edge.
// A. Get the depth and lowest common ancestor of the two nodes.
fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n - 1],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }

    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for &(x, y) in xy.iter() {
        tree[x].push(y);
        tree[y].push(x);
    }

    let mut depths: Vec<usize> = vec![0; n];
    let mut parents: Vec<usize> = vec![usize::MAX; n];

    // BFS to find parent and depth for each node
    let mut q_bfs = std::collections::VecDeque::new();
    q_bfs.push_back((0, usize::MAX, 0));
    while let Some((v, p, d)) = q_bfs.pop_front() {
        parents[v] = p;
        depths[v] = d;
        for &next in tree[v].iter() {
            if next != p {
                q_bfs.push_back((next, v, d + 1));
            }
        }
    }

    let doubling: Vec<Vec<usize>> = get_doubling(&parents);

    for &(node_a, node_b) in ab.iter() {
        let lca = lowest_common_ancestor(node_a, node_b, &doubling, &depths);
        let ans = depths[node_a] + depths[node_b] + 1 - 2 * depths[lca];
        println!("{}", ans);
    }
}

fn get_doubling(parents: &Vec<usize>) -> Vec<Vec<usize>> {
    let n: usize = parents.len();
    let mut doubling: Vec<Vec<usize>> = vec![vec![usize::MAX; n]; 20];
    for v in 0..n {
        doubling[0][v] = parents[v];
    }

    for phase in 1..20 {
        for v in 0..n {
            if doubling[phase - 1][v] == usize::MAX {
                doubling[phase][v] = usize::MAX;
            } else {
                doubling[phase][v] = doubling[phase - 1][doubling[phase - 1][v]];
            }
        }
    }
    doubling
}

fn lowest_common_ancestor(
    node_a: usize,
    node_b: usize,
    doubling: &Vec<Vec<usize>>,
    depths: &Vec<usize>,
) -> usize {
    let mut v1: usize = node_a;
    let mut v2: usize = node_b;

    // Make sure v1 is deeper than v2
    if depths[v1] < depths[v2] {
        std::mem::swap(&mut v1, &mut v2);
    }
    let depth_diff: usize = depths[v1] - depths[v2];

    // Align depth
    for phase in 0..20 {
        if (depth_diff >> phase) & 1 == 1 {
            v1 = doubling[phase][v1];
        }
    }
    // if the nodes are the same, it is the LCA
    if v1 == v2 {
        return v1;
    }

    // Binary lifting to find LCA
    for phase in (0..20).rev() {
        if doubling[phase][v1] != doubling[phase][v2] {
            v1 = doubling[phase][v1];
            v2 = doubling[phase][v2];
        }
    }
    doubling[0][v1]
}
