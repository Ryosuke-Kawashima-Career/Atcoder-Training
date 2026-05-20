# ABC 451 F: Dynamic Bipartite Clustering

## Question

Find the minimum number of black nodes of the bipartite graph.
Judge this after every query of adding edges.

## Key Points

Connected nodes are managed by Union-Find (DSU) with Splitting Vertices(頂点倍化).

## Model Answer

```rust
use proconio::{input, marker::Usize1};

// A highly optimized Bipartite Union-Find (DSU) to track dynamic odd cycles and colors
struct BipartiteDSU {
    parent: Vec<usize>,
    color: Vec<i32>,
    size: Vec<[usize; 2]>,
    is_bipartite: bool,
    pub total_min_black: usize,
}

impl BipartiteDSU {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let mut size = vec![[0, 0]; n];
        
        for i in 0..n {
            parent[i] = i;
            // Every isolated vertex initially belongs to color "0" of its own standalone component (Black/White parity)
            size[i][0] = 1;
            size[i][1] = 0;
        }
        
        BipartiteDSU {
            parent,
            color: vec![0; n],
            size,
            is_bipartite: true,
            total_min_black: 0,
        }
    }

    // Path compression that dynamically ripples the color offset up relative to the root component
    fn find(&mut self, i: usize) -> (usize, i32) {
        if self.parent[i] == i {
            (i, 0)
        } else {
            let p = self.parent[i];
            let (root, c) = self.find(p);
            self.parent[i] = root;
            self.color[i] ^= c; // Update absolute offset structurally
            (root, self.color[i])
        }
    }

    // Attempt to drop an edge between independent or existing cluster components
    fn union(&mut self, i: usize, j: usize) {
        if !self.is_bipartite { return; } // Once dead, always dead computationally
        
        let (root_i, c_i) = self.find(i);
        let (root_j, c_j) = self.find(j);

        if root_i == root_j {
            // They are already strictly chained within the exact same component structure
            if c_i == c_j {
                // If they map identically to the same exact color relative to root, forming a 1-edge breaks bipartiteness!
                self.is_bipartite = false;
            }
        } else {
            // We want c_i ^ c_edge ^ c_j to mathematically equate to 1 (ensuring different colors relative)
            let c_edge = 1 ^ c_i ^ c_j;
            
            let mut ri = root_i;
            let mut rj = root_j;
            
            // Union By Size (strictly mapping smaller segments functionally into the massive networks)
            if self.size[ri][0] + self.size[ri][1] < self.size[rj][0] + self.size[rj][1] {
                std::mem::swap(&mut ri, &mut rj);
            }
            
            // Revert their previous actively tracked isolated mathematical minimum values out
            self.total_min_black -= self.size[ri][0].min(self.size[ri][1]);
            self.total_min_black -= self.size[rj][0].min(self.size[rj][1]);
            
            // Absorb the smaller root dimension completely into the larger mathematically
            self.parent[rj] = ri;
            self.color[rj] = c_edge;
            
            if c_edge == 0 {
                // Their assigned colors perfectly/identically overlap cleanly natively
                self.size[ri][0] += self.size[rj][0];
                self.size[ri][1] += self.size[rj][1];
            } else {
                // The incoming child group functionally needs their internally tracked sizes inverted
                self.size[ri][0] += self.size[rj][1];
                self.size[ri][1] += self.size[rj][0];
            }
            
            // Register their shiny new merged Minimum constraint back permanently
            self.total_min_black += self.size[ri][0].min(self.size[ri][1]);
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        // Since input constraints guarantee strictly 1-based Node IDs, 
        // the `Usize1` dynamically drops them directly into safe 0-based rust arrays.
        queries: [(Usize1, Usize1); q],
    }
    
    let mut dsu = BipartiteDSU::new(n);
    
    for (u, v) in queries {
        dsu.union(u, v);
        
        if dsu.is_bipartite {
            println!("{}", dsu.total_min_black);
        } else {
            // Once an Odd graph loop exists, it physically poisons the entire simulation
            println!("-1"); 
        }
    }
}
```

---

### Walk Through with a Step by Step Explanation

Running $Q$ massive queries inside 2.0s means calculating full Breadth First Searches (BFS) repeatedly drops dead from Time Limit Exceeded (TLE). We strictly resort to mathematically tracking the graph components inside a **Union Find (DSU)** data structure dynamically.

1. **Building the Parity State Machine**
Rather than just keeping track of "who belongs to what tree group", we construct a customized `BipartiteDSU`. Every single node now dynamically returns two properties upon `find()`:
   - What core Component Root network it is physically housed in.
   - A theoretical color boolean offset: Is this node mathematically the Same `(0)` or the Opposite `(1)` relative color structure specifically to its exact Root Parent?

2. **Dropping Edges internally vs externally**
When joining nodes $i$ and $j$, if `find()` confirms they already belong to the same parent tree root, we evaluate their internal colors globally. If their colors natively match identically relative to the root (`c_i == c_j`), generating a direct bridge physically spanning them universally constitutes an **Odd Cycle**, instantaneously breaking the graph mathematically (`self.is_bipartite = false`).

3. **Merging Graphs dynamically (`Union By Size`)**
If they belong to totally different fragmented components, we must forcefully merge their massive roots together. But one side might explicitly need all their stored colors systematically inverted to remain compliant!
We easily track their internal populations separately `[White Count, Black Count]`. If inverted seamlessly, we physically flip the counter assignments: `[Black Count, White Count]` mathematically absorbed perfectly mapped right into the master root array!

4. **Scoring the Matrix Minimum**
Any physically isolated bipartite component functionally lets us pick whichever side we want (White vs Black partition) directly as "Black". So for $C$ broken parts independently, the ultimate minimum count answers perfectly to exactly $\sum \min(White\_Count_{C_{x}}, Black\_Count_{C_{x}})$. We dynamically recalculate this mathematical value simply adding/subtracting overlapping changes linearly every exact time `union()` is triggered!

---

### How to come up with the solution

When modeling intense Graph Topology queries across dense structures:

- **Look out for "Is it possible to paint 2 colors..." markers**: Any sequence questioning 2-Colors on Graphs exclusively screams searching for "Bipartite / Absence of Odd Cycles".
- **Identify Dynamic Updates**: Usually, finding if a static graph is Bipartite is an automatic $O(N)$ BFS. However, asking you to strictly calculate it **every single time a line is added** sequentially immediately limits standard Arrays algorithms.
- **Reach for the Parity DSU Tool**: The moment "Edges dynamically attach" meets an absolute condition mathematically tracking "Parities/Alternating Status" on Nodes, construct a **Weighted Union Find / Disjoint Set** structure! It solves connection grouping while maintaining internal distance math constraints with mindbendingly fast limits inside perfectly flat array allocations!
