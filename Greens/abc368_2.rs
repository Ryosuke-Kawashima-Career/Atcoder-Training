use proconio::{input, marker::Usize1};
use std::collections::HashSet;
// ABC368 D: Minimum Steiner Tree
// Q. Find the minimum sized tree that contains all the vertices in v.
// A. Pruning of simple paths from leaves not in target set.
// Solution: Iterative Leaf Pruning (Kahn's Algorithm style) for DAG (Directed Acyclic Graph)
fn main() {
    input! {n: usize, k: usize, ab: [(Usize1, Usize1); n-1], v: [Usize1; k]}
    // Build adjacency list
    let mut tree = vec![HashSet::new(); n];
    for &(a, b) in &ab {
        tree[a].insert(b);
        tree[b].insert(a);
    }

    // Set of target vertices for quick lookup
    let targets: HashSet<usize> = v.into_iter().collect();

    // Identify initial leaves (degree 1) that are NOT targets
    let mut queue = Vec::new();

    // Initial scan
    for i in 0..n {
        if tree[i].len() == 1 && !targets.contains(&i) {
            queue.push(i);
        }
    }

    let mut ans = n;

    // Process queue for cascading pruning
    while let Some(leaf) = queue.pop() {
        // leaf is removed.
        ans -= 1;

        // Find the neighbor to remove edge
        if let Some(&neighbor) = tree[leaf].iter().next() {
            // Remove edge (leaf, neighbor)
            tree[neighbor].remove(&leaf);

            // Check if neighbor becomes a candidate for removal
            if tree[neighbor].len() == 1 && !targets.contains(&neighbor) {
                queue.push(neighbor);
            }
        }
    }

    println!("{}", ans);
}
