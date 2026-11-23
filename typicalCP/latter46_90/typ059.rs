use proconio::{input, marker::Usize1};
// Typ059
// Q. In a toplogically ordered DAG, judge each query as to whether Node A can reach Node B.
// A. Digit represents Query !!!!
const BATCH_SIZE: usize = 64;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        xy: [(Usize1, Usize1); m],
        queries: [(Usize1, Usize1); q],
    }

    // Because X_i < Y_i for all edges, vertices 0..n are already in topological order!
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(from, to) in &xy {
        graph[from].push(to);
    }

    // dp[v]: bit `bit` is 1 iff vertex `v` is reachable from `batch[bit].0`
    let mut dp: Vec<u64> = vec![0; n];

    for batch in queries.chunks(BATCH_SIZE) {
        // dp[node] = 0b10 means the 0-th query is NOT reachable from the node-th vertex and the 1-st query is reachable from the node-th vertex.
        // 1. Reset `dp` to all 0s for the new batch
        dp.fill(0);

        // 2. Initialize `dp` with starting vertices in the current batch
        for (query_index, &(start, _goal)) in batch.iter().enumerate() {
            dp[start] |= 1u64 << query_index;
        }

        // Remember the nodes are topologically ordered and each digit represents a query!!!
        for v in 0..n {
            if dp[v] == 0 {
                continue;
            }
            for &next in &graph[v] {
                dp[next] |= dp[v];
            }
        }

        for (query_index, &(_start, goal)) in batch.iter().enumerate() {
            if (dp[goal] >> query_index) & 1u64 == 1 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
