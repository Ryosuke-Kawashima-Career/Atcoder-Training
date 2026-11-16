use proconio::{input, marker::Usize1};
// Typical CP Problem 35
// Q. What is the minimum number of edges to connect vertices?
// A. Tree DP and LCA = Lowest Common Ancestor
// Doubling DP = Binary Lifting
const N_PHASE: usize = 20;
const NIL: usize = usize::MAX;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        q: usize,
    }
    let mut should_be_connected: Vec<Vec<usize>> = Vec::new();
    for _ in 0..q {
        input! {
            k: usize,
            vertices: [Usize1; k],
        }
        should_be_connected.push(vertices);
    }
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in edges.iter() {
        tree[a].push(b);
        tree[b].push(a);
    }

    let mut doubling: Vec<Vec<usize>> = vec![vec![NIL; n]; N_PHASE];
    let mut depth: Vec<usize> = vec![NIL; n];
    let mut order: Vec<usize> = vec![0; n];
    // timer := ID for calculating the depth
    let mut timer: usize = 0;

    depth[0] = 0;
    doubling[0][0] = 0;
    dfs_init(0, &tree, &mut doubling, &mut depth, &mut order, &mut timer);
    get_doubling(&mut doubling);

    for query in 0..q {
        let target_group: &mut Vec<usize> = &mut should_be_connected[query];
        target_group.sort_by_key(|&vertex| order[vertex]);
        let k: usize = target_group.len();
        let mut ans: usize = 0;
        for i in 0..k {
            let v: usize = target_group[i];
            let next_v: usize = target_group[(i + 1) % k];
            let lca: usize = get_lca(v, next_v, &doubling, &depth);
            ans += depth[v] - depth[lca];
        }
        println!("{}", ans);
    }
}

fn dfs_init(
    start: usize,
    tree: &Vec<Vec<usize>>,
    doubling: &mut Vec<Vec<usize>>,
    depth: &mut Vec<usize>,
    order: &mut Vec<usize>,
    timer: &mut usize,
) {
    order[start] = *timer;
    *timer += 1;
    for &next in tree[start].iter() {
        if depth[next] == NIL {
            depth[next] = depth[start] + 1;
            doubling[0][next] = start;
            dfs_init(next, tree, doubling, depth, order, timer);
        }
    }
}

fn get_doubling(doubling: &mut Vec<Vec<usize>>) {
    let n: usize = doubling[0].len();
    for phase in 1..N_PHASE {
        for v in 0..n {
            // 2^phase-th parent of v = 2^(phase-1)-th parent of [2^(phase-1)-th parent of v]
            doubling[phase][v] = doubling[phase - 1][doubling[phase - 1][v]];
        }
    }
}

fn get_lca(v1: usize, v2: usize, doubling: &Vec<Vec<usize>>, depth: &Vec<usize>) -> usize {
    let mut ancestor_deeper: usize = v1;
    let mut ancestor_higher: usize = v2;
    if depth[ancestor_deeper] < depth[ancestor_higher] {
        std::mem::swap(&mut ancestor_deeper, &mut ancestor_higher);
    }

    // 1. Lift the deeper vertex to the same depth as the higher vertex
    let diff: usize = depth[ancestor_deeper] - depth[ancestor_higher];
    for phase in 0..N_PHASE {
        if (diff >> phase) & 1 == 1 {
            ancestor_deeper = doubling[phase][ancestor_deeper];
        }
    }

    if ancestor_deeper == ancestor_higher {
        return ancestor_deeper;
    }

    // 2. Lift both vertices together until right below their LCA
    for phase in (0..N_PHASE).rev() {
        if doubling[phase][ancestor_deeper] != doubling[phase][ancestor_higher] {
            ancestor_deeper = doubling[phase][ancestor_deeper];
            ancestor_higher = doubling[phase][ancestor_higher];
        }
    }

    doubling[0][ancestor_deeper]
}
