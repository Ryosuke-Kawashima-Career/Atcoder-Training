use proconio::{input, marker::Usize1};
// abc373d
// Q. x --(w)--> y
// A. Potential Based Weighted Disjoint Union (DSU)

fn main() {
    input! {n: usize, m: usize, uvw: [(Usize1, Usize1, i64); m]}
    let mut pdsu = PotentialDSU::new(n);
    for &(v1, v2, weight) in uvw.iter() {
        pdsu.union(v1, v2, weight);
    }
    for v in 0..n {
        print!("{} ", pdsu.get_potential(v));
    }
    println!("");
}

#[derive(Debug)]
struct PotentialDSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    // potential[x] := V(x) - V(parent(x))
    potential: Vec<i64>,
}

impl PotentialDSU {
    fn new(n: usize) -> Self {
        let parent: Vec<usize> = (0..n).collect();
        let size: Vec<usize> = vec![1; n];
        let potential: Vec<i64> = vec![0; n];
        Self {
            parent,
            size,
            potential,
        }
    }

    fn find(&mut self, node: usize) -> usize {
        /* Returns the node's parent
        Recursively gets its parent node
        */
        if self.parent[node] == node {
            return node;
        }
        let root: usize = self.find(self.parent[node]);
        self.potential[node] += self.potential[self.parent[node]];
        // Path compression
        self.parent[node] = root;
        return root;
    }

    fn diff(&mut self, node1: usize, node2: usize) -> Option<i64> {
        /* Return the result V(node2) - V(node1) if the two nodes are in the same set
        The output is None if there is some contradiction.
        */
        let root1: usize = self.find(node1);
        let root2: usize = self.find(node2);
        if root1 == root2 {
            return Some(self.potential[node2] - self.potential[node1]);
        } else {
            return None;
        }
    }

    fn union(&mut self, node1: usize, node2: usize, weight: i64) -> bool {
        /* Connects x and y with relation V(y) - V(x) = W
        Return true if the edge is new, and false if the edge is contradictory.
        */
        let mut root1: usize = self.find(node1);
        let mut root2: usize = self.find(node2);

        if root1 == root2 {
            return self.diff(node1, node2) == Some(weight);
        }
        // V(node2) - V(node1) = weight
        // V(node2) = V(root2) + potential[node2]
        // V(node1) = V(root1) + potential[node1]
        // V(root2) + potential[node2] - V(root1) - potential[node1] = weight
        // V(root2) - V(root1) = weight + potential[node1] - potential[node2]
        let mut new_weight: i64 = weight + self.potential[node1] - self.potential[node2];
        if self.size[root1] < self.size[root2] {
            std::mem::swap(&mut root1, &mut root2);
            new_weight = -new_weight;
        }
        // Attach Smaller root to Bigger root
        self.parent[root2] = root1;
        self.size[root1] += self.size[root2];
        self.potential[root2] = new_weight;
        return true;
    }

    fn get_potential(&mut self, node: usize) -> i64 {
        let _root: usize = self.find(node);
        return self.potential[node];
    }
}

