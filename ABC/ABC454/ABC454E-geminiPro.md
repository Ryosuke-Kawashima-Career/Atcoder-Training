# Hamiltonian Path in a Grid

## Problem Statement

Get a Hamiltonian path from $(1,1)$ to $(N,N)$ in a grid of even $N$, skipping the cell $(A,B)$.

---

### Model Answer

```rust
use proconio::{input, marker::Usize1};

fn solve() {
    input! {
        n: usize,
        a: Usize1,
        b: Usize1,
    }

    // A Hamiltonian path from (1,1) to (N,N) in a grid of even N must start and end on the same parity.
    // The missing point must be of odd parity.
    if n % 2 != 0 || (a + b) % 2 == 0 {
        println!("No");
        return;
    }

    println!("Yes");
    let mut votqi = String::with_capacity(n * n);
    let h = b / 2;

    for s in 0..(n / 2) {
        if s < h {
            // Traverse from Top-Left to Top-Right
            for _ in 0..(n - 1) { votqi.push('D'); }
            votqi.push('R');
            for _ in 0..(n - 1) { votqi.push('U'); }
        } else if s > h {
            // Traverse from Bottom-Left to Bottom-Right
            for _ in 0..(n - 1) { votqi.push('U'); }
            votqi.push('R');
            for _ in 0..(n - 1) { votqi.push('D'); }
        } else {
            // The hole is dynamically weaved over top-to-bottom
            let l = 2 * s;
            let r = 2 * s + 1;
            let mut curr_c = l;
            for row in 0..n {
                if row < a {
                    if row % 2 == 0 {
                        votqi.push('R');
                        curr_c = r;
                    } else {
                        votqi.push('L');
                        curr_c = l;
                    }
                } else if row > a {
                    if row % 2 == 0 {
                        votqi.push('L');
                        curr_c = l;
                    } else {
                        votqi.push('R');
                        curr_c = r;
                    }
                }
                
                if row < n - 1 {
                    votqi.push('D');
                }
            }
        }
        
        // Transition to the next 2-column wide strip 
        if s < n / 2 - 1 {
            votqi.push('R');
        }
    }
    println!("{}", votqi);
}

fn main() {
    input! { t: usize }
    for _ in 0..t {
        solve();
    }
}
```

### Walk-through with a Step by Step Explanation

In grid path puzzles like these, it is a great idea to group the columns into chunks. In this problem, we split our $N \times N$ grid into $N/2$ vertical strips, each of width 2:
Columns $[0,1]$, Columns $[2,3]$, ..., Columns $[N-2, N-1]$.
The flaw in your pre-existing code was how the grid routing was performed: although weaving column-by-column handles standard areas, doing normal U-turning snakes over horizontal strips across the full board length ends up reaching $(0, N-1)$ instead of $(N-1, N-1)$ when there are an even number of vertical strips. In addition, fixing logic near the hole incorrectly disrupted horizontal logic for the remainder of the strip.

To accurately address parity offsets cleanly, we organize our $2 \times N$ column strips into three phases, based on bounding where our white `hole` cell is. Let the strip containing the hole be `H = b / 2`.

1. **Before the Hole (`s < H`)**: Since the hole hasn't been encountered yet, we perform **Top-to-Top U-turns**. We enter at the Top-Left corner of a strip $(0, 2s)$, move strictly down until hitting the bottom, take one step Right, and then step continuously Up. This drops us directly at $(0, 2s+1)$ in preparation to simply move Right into the next strip while remaining at the ceiling row.
2. **At the Hole (`s == H`)**: Because the cell $(A, B)$ must be skipped and contains odd parity `(a+b)%2 == 1`, this strip drops 1 node out of its $2N$ sub-mesh—giving it exactly an odd $2N-1$ valid traversal cells. Since any odd sequence within a bipartite mesh will invert parity exactly an even number of times, it starts at its Top-Left and will naturally end exactly at its Bottom-Right node! We construct this logic to bounce Top-to-Bottom. To skip the hole, you mirror the direction of horizontal weave (`R` $\to$ `L`) exactly for all subsequent steps *under* the hole row, rendering crisscross bugs eradicated.
3. **After the Hole (`s > H`)**: Since the strip belonging to the hole exited cleanly into the right bottom cell, all sequential operations need to start at Bottom-Left instead. Here, we flip the rule and perform **Bottom-to-Bottom U-turns** (traverse UP, RIGHT, DOWN). This perfectly propagates our bottom coordinate straight across into the finish destination: the globally required exit cell, $(N-1, N-1)$.

### How to Come Up with the Solution

1. **Identify Checkers/Parities**: The very first conceptual jump is to color the field similar to staggered checkerboards based on $(row + col) \% 2$. Start cell $(0,0)$ provides $0 \% 2 == 0$ (Even/Black). Ending cell $(N-1, N-1)$ provides $(N-1 + N-1) \% 2 \implies 2*(N-1) \% 2 = 0$ (also Even/Black).
2. **Identify Node Imbalances**: For even $N$, the overall grid contains identically colored start and end vertices but contains Hamiltonian subpaths demanding the grid to flip parity per step. If equal amounts of colors existed natively, Hamiltonian routing between same color starts/ends is strictly impossible. As a sub-solution requirement, we are effectively slicing $1$ node. Removing $1$ White Node (an Odd Sum Index) breaks the balance properly, permitting such routing traversal length. Thus the hole `(a + b) % 2 == 1` is physically required.
3. **Partition Clean Groupings**: The overarching puzzle becomes a system of independent sub-graph linkages. Grouping out to vertical strips of $N \times 2$ divides the challenge into 3 separate behaviors where logic becomes rigidly scalable since any grid strip without hole exceptions takes symmetric U-Turns cleanly returning back to its exact initial latitude.
