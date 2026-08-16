# 🏆 Competitive Programming Algorithm Relearning Problem Set

This document compiles all **17 core algorithms and data structures** featured in our sessions. Each section includes a formal **Problem Statement**, **Constraints**, **Input/Output Examples**, **Core Mechanism & Invariants**, and a **Relearning Checklist**.

---

## 📑 Table of Contents
1. [String: Z-Algorithm](#1-string-z-algorithm)
2. [String: KMP Algorithm & LPS Array](#2-string-kmp-algorithm--lps-array)
3. [Game Theory: Sprague-Grundy Theorem & MEX DP](#3-game-theory-sprague-grundy-theorem--mex-dp)
4. [Data Structure: Disjoint-Interval MEX Set](#4-data-structure-disjoint-interval-mex-set)
5. [Graph / Data Structure: Potential-Based Weighted Union-Find (DSU)](#5-graph--data-structure-potential-based-weighted-union-find-dsu)
6. [Tree Algorithm: Lowest Common Ancestor (LCA) with Binary Lifting](#6-tree-algorithm-lowest-common-ancestor-lca-with-binary-lifting)
7. [Grid / 2-Pointers: 2D Column Prefix Sums & Shakutori Subgrid Count](#7-grid--2-pointers-2d-column-prefix-sums--shakutori-subgrid-count)
8. [Dynamic Programming: Digit DP with Bitmask & Modulo State](#8-dynamic-programming-digit-dp-with-bitmask--modulo-state)
9. [Dynamic Programming: Kadane's Algorithm & Range Recovery](#9-dynamic-programming-kadanes-algorithm--range-recovery)
10. [Data Structure: High-Performance Arena Doubly-Linked List](#10-data-structure-high-performance-arena-doubly-linked-list)
11. [Data Structure / Geometry: 2D Offline Rectangle Sum with Sweepline & BIT](#11-data-structure--geometry-2d-offline-rectangle-sum-with-sweepline--bit)
12. [Permutation Algorithm: Lexicographically Next Permutation](#12-permutation-algorithm-lexicographically-next-permutation)
13. [Sweepline / Combinatorics: Interval Boundary State Classification & Binomials](#13-sweepline--combinatorics-interval-boundary-state-classification--binomials)
14. [Graph / Ad-Hoc: Anchor Candidate Pruning via Pigeonhole Principle](#14-graph--ad-hoc-anchor-candidate-pruning-via-pigeonhole-principle)
15. [Data Structure: Sparse Table for Static $\mathcal{O}(1)$ Range Minimum Queries](#15-data-structure-sparse-table-for-static-mathcalo1-range-minimum-queries)
16. [Network Flow: Ford-Fulkerson, Edmonds-Karp & Dinic Algorithms](#16-network-flow-ford-fulkerson-edmonds-karp--dinic-algorithms)
17. [Bit Manipulation / Data Structure: Multi-Scale XOR Fenwick Trees with Shifts](#17-bit-manipulation--data-structure-multi-scale-xor-fenwick-trees-with-shifts)

---

## 1. String: Z-Algorithm
- [ ] Implement `get_z_array(s: &str) -> Vec<usize>` in $\mathcal{O}(|S|)$.
- [ ] Solve exact substring pattern matching in $\mathcal{O}(|P| + |T|)$ using separator `"$"` or `"\0"`.

### Problem Statement
Given a string $S$ of length $N$, compute the Z-array $Z$, where $Z[i]$ is the length of the longest common prefix (LCP) between $S$ and the suffix of $S$ starting at index $i$ ($S[i \dots N-1]$). By definition, $Z[0] = N$ (or $0$).

### Key Invariant
Maintain the rightmost Z-box $[L, R]$ such that $S[L \dots R] = S[0 \dots R - L]$.
- If $i \le R$: initialize $Z[i] = \min(R - i + 1, Z[i - L])$.
- Expand characters naively beyond $R$ and update $[L, R]$.

---

## 2. String: KMP Algorithm & LPS Array
- [ ] Implement Longest Prefix Suffix (`get_lps_array(p: &str) -> Vec<usize>`) in $\mathcal{O}(|P|)$.
- [ ] Implement text search without backtracking the text pointer in $\mathcal{O}(|T|)$.

### Problem Statement
Given a text $T$ and a pattern $P$, find all 0-based starting indices where $P$ occurs in $T$.

### Key Invariant
$LPS[i]$ stores the length of the longest proper prefix of $P[0 \dots i]$ that is also a suffix of $P[0 \dots i]$. When a mismatch occurs at $P[j]$, fallback to $j \gets LPS[j - 1]$ without moving the text pointer $i$.

---

## 3. Game Theory: Sprague-Grundy Theorem & MEX DP
- [ ] Prove state transition closedness ($\sum w_i + b_i \le 200$).
- [ ] Implement 2D bottom-up DP table for Grundy numbers with Minimum Excluded value (MEX).

### Problem Statement
Two players play a game on piles of white ($w$) and black ($b$) stones with allowed operations:
1. Remove 1 white stone ($w \to w-1$).
2. Remove $k$ black stones where $1 \le k \le w$ ($b \to b-k$).
3. Exchange 1 black stone for 1 white stone ($w \to w+1, b \to b-1$).
Determine whether the first player wins for independent game piles.

### Key Invariant
$$\text{Grundy}(w, b) = \text{MEX}\left( \{ \text{Grundy}(w-1, b) \} \cup \{ \text{Grundy}(w, b-k) \mid 1 \le k \le w \} \cup \{ \text{Grundy}(w+1, b-1) \mid b \ge 1 \} \right)$$
Evaluate by outer loop on $b$ and inner loop on $w$.

---

## 4. Data Structure: Disjoint-Interval MEX Set
- [ ] Implement `MexSet` using `BTreeSet<(usize, usize)>` to maintain non-overlapping intervals.
- [ ] Implement `add(x)`, `remove(x)`, and `get_mex() -> usize` in $\mathcal{O}(\log N)$.

### Problem Statement
Maintain a multiset of non-negative integers supporting:
- Add element $x$.
- Remove element $x$.
- Query the smallest non-negative integer not present in the set (MEX).

### Key Invariant
Store continuous present intervals $[l, r)$.
- When querying MEX, inspect the interval covering $0$. If $[0, r)$ exists, $\text{MEX} = r$; otherwise $\text{MEX} = 0$.

---

## 5. Graph / Data Structure: Potential-Based Weighted Union-Find (DSU)
- [ ] Implement `find`, `union(u, v, w)`, and `diff(u, v) -> Option<i64>`.
- [ ] Correctly handle size-based root swapping with potential weight negation: `new_weight = -new_weight`.

### Problem Statement
Maintain a system of potential relations $V(y) - V(x) = W$ on a graph with $N$ vertices and $M$ relations. Support online queries: determine if $V(b) - V(a)$ is uniquely determined, and return its value.

### Key Invariant
$\text{potential}[x] = V(x) - V(\text{parent}(x))$.
- Path Compression: $\text{potential}[x] \gets \text{potential}[x] + \text{potential}[\text{old\_parent}]$.
- Union: $V(\text{root}_2) - V(\text{root}_1) = W + \text{potential}[x] - \text{potential}[y]$.

---

## 6. Tree Algorithm: Lowest Common Ancestor (LCA) with Binary Lifting
- [ ] Precompute $2^k$-ancestor table `doubling[k][v]` in $\mathcal{O}(N \log N)$.
- [ ] Answer LCA queries in $\mathcal{O}(\log N)$ by depth alignment and simultaneous lifting.

### Problem Statement
Given a tree with $N$ vertices rooted at $0$, process $Q$ queries: compute the lowest common ancestor of vertices $u$ and $v$.

### Key Invariant
1. Lift the deeper node until $\text{depth}[u] == \text{depth}[v]$ using the binary representation of $\Delta d$.
2. If $u == v$, return $u$.
3. Simultaneously lift $u$ and $v$ from $k = 19 \dots 0$: if `doubling[k][u] != doubling[k][v]`, move $u \gets \text{doubling}[k][u]$ and $v \gets \text{doubling}[k][v]$.
4. Return `doubling[0][u]`.

---

## 7. Grid / 2-Pointers: 2D Column Prefix Sums & Shakutori Subgrid Count
- [ ] Compute vertical 1D column prefix sums in $\mathcal{O}(H \cdot W)$.
- [ ] Iterate over all row pairs $(r_1, r_2)$ and count subgrids with sum $K$ using two-pointers in $\mathcal{O}(H^2 W)$.

### Problem Statement
Given an $H \times W$ grid of $0$s and $1$s, find the number of rectangular subgrids whose elements sum to $K$. Constraints: $H, W \le 500$.

### Common Pitfall
Do **not** mix vertical prefix sums with horizontal prefix sums during preprocessing:
$$\text{vertical}[i][j] = \text{grid}[i][j] + \text{vertical}[i-1][j]$$

---

## 8. Dynamic Programming: Digit DP with Bitmask & Modulo State
- [ ] Set up 4D DP state: `dp[is_less][is_leading_zero][mod3][mask]`.
- [ ] Transition over digits $0 \dots \text{max\_d}$ for large $N < 10^{500}$.

### Problem Statement
Count positive integers $x \le N$ satisfying **exactly one** of the three conditions:
1. $x$ is a multiple of 3.
2. The decimal representation contains '3'.
3. The decimal representation uses exactly 3 distinct digit characters.

---

## 9. Dynamic Programming: Kadane's Algorithm & Range Recovery
- [ ] Implement $\mathcal{O}(N)$ time and $\mathcal{O}(1)$ space maximum subarray sum.
- [ ] Track indices `[start, end)` using a temporary start pointer `temp_start`.

### Key Invariant
$$\text{current\_max}[i] = \max(\text{current\_max}[i-1] + A[i], A[i])$$
- If $\text{current\_max} < 0$, set `temp_start = i` (restart).
- If $\text{current\_max} > \text{global\_max}$, update `global_start = temp_start`, `global_end = i + 1`.

---

## 10. Data Structure: High-Performance Arena Doubly-Linked List
- [ ] Avoid lifetime / borrow-checker pitfalls in Rust by using `Vec<Node>` and `Option<usize>` index pointers.
- [ ] Implement node insertion, deletion, bidirectional linking, and traversal.

### Key Structure
```rust
struct Node {
    id: usize,
    next: Option<usize>,
    prev: Option<usize>,
}
```

---

## 11. Data Structure / Geometry: 2D Offline Rectangle Sum with Sweepline & BIT
- [ ] Decompose each 2D range query $[l, r) \times [d, u)$ into 2 $X$-axis sweep events ($x = l$ with $-1$, $x = r$ with $+1$).
- [ ] Apply coordinate compression on $Y$-coordinates and maintain a 1D Fenwick Tree.
- [ ] Achieve $\mathcal{O}((N + Q) \log(N + Q))$ overall complexity.

---

## 12. Permutation Algorithm: Lexicographically Next Permutation
- [ ] Implement generic `next_permutation<T: Ord>(a: &mut [T]) -> bool` in $\mathcal{O}(N)$.

### 4 Step Algorithm
1. Find largest $i$ such that $a[i] < a[i + 1]$ from right. If none, reverse array and return `false`.
2. Find largest $j > i$ such that $a[j] > a[i]$ from right.
3. `a.swap(i, j)`.
4. `a[i + 1..].reverse()`. Return `true`.

---

## 13. Sweepline / Combinatorics: Interval Boundary State Classification & Binomials
- [ ] Precompute factorials for $\mathcal{O}(1)$ binomial coefficients $\binom{n}{r}$.
- [ ] Register boundary change events at critical points: $L_i, R_i + 1, N - R_i, N - L_i + 1$.
- [ ] Maintain state category matrix $M[aok][bok]$ in $\mathcal{O}(N)$ total transitions.

---

## 14. Graph / Ad-Hoc: Anchor Candidate Pruning via Pigeonhole Principle
- [ ] Anchor candidates using finalists of tournament 0 ($A_0, B_0$).
- [ ] Reduce partner candidates to at most 2 by inspecting any missed tournament.
- [ ] Verify pair validity in $\mathcal{O}(1)$: $c(u) + c(v) - c(u, v) == M$.

---

## 15. Data Structure: Sparse Table for Static $\mathcal{O}(1)$ Range Minimum Queries
- [ ] Precompute $2^k$ table `st[k][i]` in $\mathcal{O}(N \log N)$ for idempotent operations.
- [ ] Answer queries in $\mathcal{O}(1)$ via:
$$\text{RMQ}(L, R) = \min(\text{st}[k][L], \text{st}[k][R - 2^k + 1]) \quad \text{where } k = \lfloor \log_2(R - L + 1) \rfloor$$

---

## 16. Network Flow: Ford-Fulkerson, Edmonds-Karp & Dinic Algorithms
- [ ] Understand residual graphs, forward capacities, and backward cancellation edges.
- [ ] **Ford-Fulkerson**: DFS augmenting paths ($\mathcal{O}(F \cdot E)$).
- [ ] **Edmonds-Karp**: BFS shortest augmenting paths ($\mathcal{O}(V \cdot E^2)$).
- [ ] **Dinic's Algorithm**: BFS Level Graph + DFS Blocking Flow with current edge pointer `iter[u]` ($\mathcal{O}(V^2 E)$).

---

## 17. Bit Manipulation / Data Structure: Multi-Scale XOR Fenwick Trees with Shifts
- [ ] Map $k$-th bit validity of $(v - s)$ to periodic range query $v \bmod 2^{k+1} \in [(s + 2^k) \bmod 2^{k+1}, (s + 2^{k+1} - 1) \bmod 2^{k+1}]$.
- [ ] Handle circular wrap-around intervals with XOR Fenwick trees.
- [ ] Remove expired elements with `bits[k].toggle(s % sz)` when $s = v$.
