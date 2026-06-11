use proconio::{input, marker::Usize1};
struct DisjointSetUnion {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        let parent: Vec<usize> = (0..n).collect();
        let size: Vec<usize> = vec![1; n];
        Self {
            parent, size
        }
    }
    fn root(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }
        let root_node: usize = self.root(self.parent[node]);
        self.parent[node] = root_node; // path compression
        root_node
    }
    fn union(&mut self, node_a: usize, node_b: usize) -> bool {
        /*unites two nodes and returns true when they belong to different parents
        false when they belong to the same group
        */
        let root_a: usize = self.root(node_a);
        let root_b: usize = self.root(node_b);
        if root_a == root_b {
            return false;
        }
        match self.size[root_a].cmp(&self.size[root_b]) {
            std::cmp::Ordering::Less => self.parent[root_a] = root_b,
            std::cmp::Ordering::Greater => self.parent[root_b] = root_a,
            std::cmp::Ordering::Equal => {
                self.parent[root_b] = root_a;
                self.size[root_a] += self.size[root_b];
            }
        }
        true
    }
}
fn main() {
    input!{n: usize, m: usize, ab: [(Usize1, Usize1); m]}
    let mut ans: usize = 0;
    for unused_edge_idx in 0..m {
        let mut dsu = DisjointSetUnion::new(n);
        for edge_idx in 0..m {
            if unused_edge_idx != edge_idx {
                let (v1, v2) = ab[edge_idx];
                dsu.union(v1, v2);
            }
        }
        let root0 = dsu.root(0);
        let is_all_connected: bool = (1..n).all(|v| dsu.root(v) == root0);
        if !is_all_connected {
            ans += 1;
        }
    }
    println!("{}", ans);
}