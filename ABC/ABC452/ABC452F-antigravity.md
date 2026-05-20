# ABC452F: Inversions

## Problem Statement

You are given an array $P$ of length $N$, where $P$ is a permutation of $1, 2, \dots, N$. Find the number of pairs of indices $(i, j)$ such that $1 \le i \le j \le N$ and the number of inversions in the subarray $P[i..=j]$ is exactly $K$.

---

## Model Answer

```rust
use proconio::input;

// A standard 1-based Fenwick Tree (Binary Indexed Tree) mapped strictly for fast point-updates and range-queries
struct Fenwick {
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick {
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut i: usize, delta: i64) {
        while i < self.tree.len() {
            self.tree[i] += delta;
            // Best Practice Rust LSB extraction: avoids !i + 1 Debug Panics natively
            i += i & i.wrapping_neg(); 
        }
    }

    fn query(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }
}

// Employs a Fast Sliding-Window to map count of subarrays where Inversions <= X
fn solve_less_equal(x: i64, n: usize, p: &[usize]) -> i64 {
    // Math guard: negative inversions are mathematically impossible natively
    if x < 0 {
        return 0;
    }
    
    let mut fenwick = Fenwick::new(n);
    let mut ans = 0i64;
    let mut inv = 0i64;
    let mut l = 0;

    // Expand the upper boundary systematically
    for r in 0..n {
        let val = p[r];
        // Calculate elements currently in the window universally strictly greater than `val`
        let strictly_greater = fenwick.query(n) - fenwick.query(val);
        inv += strictly_greater;
        
        // Push the new boundary limit formally into the tracked sequence map
        fenwick.add(val, 1);

        // While inversions break limit boundaries, compress the exact bottom boundary uniquely
        while inv > x {
            let left_val = p[l];
            // Identify how many parameters natively currently exist strictly less than the dying parameter
            let strictly_less = fenwick.query(left_val - 1);
            inv -= strictly_less;
            
            // Delete the dying boundary functionally from the active mapped limits
            fenwick.add(left_val, -1);
            l += 1;
        }

        // Add universally all valid subsets ending precisely at this 'r' index natively!
        ans += (r - l + 1) as i64;
    }
    ans
}

fn main() {
    input! {
        n: usize,
        k: i64,
        p: [usize; n],
    }

    // Number of Pairs yielding exactly K inversions equates physically to:
    // f(Total subsets <= K) - f(Total Subsets <= K - 1)
    let ans = solve_less_equal(k, n, &p) - solve_less_equal(k - 1, n, &p);
    println!("{}", ans);
}
```

---

### Walk Through with a Step by Step Explanation

Solving problems calculating "exact ranges equaling exact mathematical targets natively" requires rethinking the output dynamically:

1. **Calculate Through Inequality Filtering**:
Hunting for combinations summing EXACTLY to $K$ requires jumping matrices overlapping randomly natively! We universally bypass calculating combinations strictly targeting `= K`. Instead, we design an algorithm entirely built exclusively to answer `"Total Number of subsets featuring <= X inversions"`.
Calculating `Total(<= K) - Total(<= K - 1)` perfectly isolates literally the parameters physically yielding exactly $K$!

2. **The Two-Pointer Inversions Window**:
Isolating parameters scaling to `<= X` natively grants us strict **Monotonicity**! If a window spans $l \dots r$ and records $4$ inversions, expanding the Right boundary physically `r += 1` strictly mathematically only increases or matches combinations (e.g. Inversions jump to $6$).
Immediately when $6$ breaks your designated target $X$, you physically compress the Left boundary `l += 1` systematically reducing inversions continuously until restoring exactly $\le X$ dynamically!

3. **Dynamic Fenwick Tracking (`O(log N)`)**:
Unlike standard sum arrays calculating bounds natively, tracking elements physically shrinking/growing demands dynamically registering variables mapping $< P_l$ and $> P_{r}$ instantly $O(\log N)$ speeds. For every single index evaluated during `l` compressions, querying natively against a heavily tailored unweighted Binary Indexed Tree (Fenwick) pulls isolated limits instantly cleanly across all nested configurations natively!

---

### How to Come up With the Solution

Identifying algorithms physically matching massive $O(N^2)$ tracking limits systematically breaks down relying heavily upon two algorithmic heuristics:

- **Exact Limits Equalling Ranges universally demands "At Most" Math**: Anytime isolated combinations demand mapping exact limits across dense combinations, uniformly reevaluate the function logically testing $F(Exact Target) = F(\le Target) - F(\le Target - 1)$.
- **Monotonic Windows Demand "Fenwick Trees"**: Recognizing the physical algorithm operates expanding/shrinking continuously limits arrays monotonically demands employing "Sliding Windows". Sliding Windows tracking active elements dynamically internally structurally strictly mandate `Fenwick/Segment` limits inherently bypassing arrays limits completely. Implementing custom Fenwick blocks conditionally speeds nested sweeps scaling dynamically underneath competitive Time Limit exceptions completely seamlessly!
