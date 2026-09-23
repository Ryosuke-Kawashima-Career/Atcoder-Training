# Simplex Algorithm, Slack Variables & Pivoting

A comprehensive guide and reference implementation of the Simplex Method for Linear Programming (LP), designed for competitive programming and mathematical optimization.

---

## 1. Overview of Linear Programming (LP)

A canonical Linear Program in standard inequality form is:

$$
\begin{aligned}
\text{maximize} \quad & z = \mathbf{c}^T \mathbf{x} \\
\text{subject to} \quad & A\mathbf{x} \le \mathbf{b} \\
& \mathbf{x} \ge \mathbf{0}
\end{aligned}
$$

Where:
- $\mathbf{x} \in \mathbb{R}^n$ is the vector of decision variables.
- $\mathbf{c} \in \mathbb{R}^n$ is the objective coefficient vector.
- $A \in \mathbb{R}^{m \times n}$ and $\mathbf{b} \in \mathbb{R}^m$ represent $m$ linear constraints.

The feasible region is a **convex polyhedron** (simplex/polytope). Fundamental theorem of linear programming states that if an optimal solution exists, at least one optimal solution occurs at an **extreme point (vertex)** of this polyhedron.

---

## 2. Slack Variables

### Motivation
Matrix algebra and Gaussian elimination operate on **equalities**, not inequalities. 

To convert each $\le$ constraint into an equation, we introduce a non-negative **slack variable** $s_i \ge 0$ representing the difference (leftover capacity) between the right-hand side and left-hand side:

$$\sum_{j=1}^n A_{i, j} x_j \le b_i \quad \iff \quad \sum_{j=1}^n A_{i, j} x_j + s_i = b_i, \quad s_i \ge 0$$

### Geometry of Slack Variables
- When $s_i > 0$: The point lies strictly inside the feasible half-space (the constraint is inactive / has slack).
- When $s_i = 0$: The point lies directly on the boundary hyper-plane defined by the $i$-th constraint (the constraint is tight/binding).

---

## 3. Basic vs. Non-Basic Variables

With $m$ constraints and $n$ original variables, we have $n + m$ total variables and $m$ equations:
- **Degree of freedom**: $(n + m) - m = n$.
- Set $n$ variables to $0$ $\rightarrow$ **Non-Basic Variables**.
- Solve the remaining $m$ variables uniquely $\rightarrow$ **Basic Variables** (the **Basis**).

Each choice of basis corresponds algebraically to an intersection of boundary hyperplanes, i.e., a **vertex** of the feasible region.
A basic solution is **feasible (BFS)** if all basic variables satisfy $x_i \ge 0, s_i \ge 0$.

---

## 4. Pivoting: Moving Between Vertices

Pivoting is the algebraic mechanism of walking along an edge of the polyhedron from one vertex to an adjacent, higher-value vertex.

One variable transitions into the basis (**entering variable**), and another transitions out (**leaving variable**).

### The Pivot Steps

1. **Entering Variable Selection (Pivot Column $q$)**:
   Look at the objective equation:
   $$z = z_0 + \sum_{j \in \text{Non-Basic}} \bar{c}_j x_j$$
   - If all reduced costs $\bar{c}_j \le 0$, no variable can increase $z$. The current solution is **optimal**.
   - Otherwise, pick $q$ such that $\bar{c}_q > 0$. (To avoid cycling in degenerate cases, **Bland's rule** picks the smallest index $q$ with $\bar{c}_q > 0$).

2. **Leaving Variable Selection (Pivot Row $p$ - Minimum Ratio Test)**:
   As $x_q$ increases from $0$, basic variables decrease:
   $$x_{\text{basic}, i} = \bar{b}_i - \bar{A}_{i, q} x_q \ge 0 \implies x_q \le \frac{\bar{b}_i}{\bar{A}_{i, q}} \quad (\text{for } \bar{A}_{i, q} > 0)$$
   To ensure no basic variable becomes negative, choose row $p$ minimizing:
   $$\theta = \frac{\bar{b}_p}{\bar{A}_{p, q}} = \min_{i: \bar{A}_{i, q} > 0} \left( \frac{\bar{b}_i}{\bar{A}_{i, q}} \right)$$
   - If $\bar{A}_{i, q} \le 0$ for all $i$, $x_q$ can grow indefinitely without violating feasibility $\rightarrow$ The LP is **unbounded** ($+\infty$).

3. **Gaussian Elimination (Row Operations)**:
   - Normalize the pivot row: divide row $p$ by $\bar{A}_{p, q}$.
   - Eliminate column $q$ from all other constraint rows and the objective row.

---

## 5. Walkthrough by Example

Maximize $z = 3x_1 + 2x_2$ subject to:
1. $2x_1 + x_2 \le 100$
2. $x_1 + x_2 \le 80$
3. $x_1, x_2 \ge 0$

### Adding Slack Variables:
$$
\begin{aligned}
s_1 &= 100 - 2x_1 - x_2 \\
s_2 &= 80 - x_1 - x_2 \\
z   &= 0 + 3x_1 + 2x_2
\end{aligned}
$$

### Iteration 1:
- **Current Basis**: $\{s_1 = 100, s_2 = 80\}$, Non-basic: $\{x_1 = 0, x_2 = 0\}$. Current $z = 0$. Vertex: $(0, 0)$.
- **Entering**: $x_1$ (highest positive objective coefficient: $+3$).
- **Minimum Ratio**:
  - $s_1: 100 / 2 = 50$
  - $s_2: 80 / 1 = 80$
  - Minimum is $50 \implies s_1$ leaves the basis.
- **Pivot**: Express $x_1 = 50 - 0.5s_1 - 0.5x_2$ and substitute:
  $$
  \begin{aligned}
  x_1 &= 50 - 0.5s_1 - 0.5x_2 \\
  s_2 &= 30 + 0.5s_1 - 0.5x_2 \\
  z   &= 150 - 1.5s_1 + 0.5x_2
  \end{aligned}
  $$
  New vertex: $(50, 0)$, $z = 150$.

### Iteration 2:
- **Entering**: $x_2$ (coefficient is $+0.5 > 0$).
- **Minimum Ratio**:
  - $x_1: 50 / 0.5 = 100$
  - $s_2: 30 / 0.5 = 60$
  - Minimum is $60 \implies s_2$ leaves the basis.
- **Pivot**: Express $x_2 = 60 + s_1 - 2s_2$ and substitute:
  $$
  \begin{aligned}
  x_1 &= 20 - s_1 + s_2 \\
  x_2 &= 60 + s_1 - 2s_2 \\
  z   &= 180 - s_1 - s_2
  \end{aligned}
  $$
  All reduced costs are negative ($-1 \le 0, -1 \le 0$). **Optimal reached!**
  - $x_1 = 20, x_2 = 60$, optimal value $z = 180$.

---

## 6. Self-Contained Rust Implementation

File location: `working/Algorithms/simplex.rs`

```rust
const EPS: f64 = 1e-9;
const INF: f64 = 1e18;

#[derive(Debug, PartialEq)]
pub enum SimplexResult {
    Optimal(f64, Vec<f64>), // (optimal_value, solution_vector)
    Infeasible,
    Unbounded,
}

pub struct Simplex {
    m: usize,               // Number of constraints
    n: usize,               // Number of variables
    t: Vec<Vec<f64>>,       // Tableau: (m + 2) x (n + 1)
    basis: Vec<usize>,      // Basic variable index for each row 0..m
    non_basis: Vec<usize>,  // Non-basic variable index for each column 0..n
}

impl Simplex {
    /// Solves:
    /// maximize    c^T x
    /// subject to  A x <= b, x >= 0
    pub fn new(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> Self {
        let m = a.len();
        let n = c.len();

        let mut t = vec![vec![0.0; n + 1]; m + 2];
        let mut basis = vec![0; m];
        let mut non_basis = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                t[i][j] = a[i][j];
            }
            t[i][n] = b[i];
            basis[i] = n + i;
        }

        for j in 0..n {
            t[m][j] = -c[j];
            non_basis[j] = j;
        }

        Self { m, n, t, basis, non_basis }
    }

    fn pivot(&mut self, p: usize, q: usize) {
        let inv = 1.0 / self.t[p][q];

        for i in 0..=self.m + 1 {
            if i != p {
                let factor = self.t[i][q] * inv;
                if factor.abs() > EPS {
                    for j in 0..=self.n {
                        if j != q {
                            self.t[i][j] -= factor * self.t[p][j];
                        }
                    }
                    self.t[i][q] = -factor;
                } else {
                    self.t[i][q] = 0.0;
                }
            }
        }

        for j in 0..=self.n {
            if j != q {
                self.t[p][j] *= inv;
            }
        }
        self.t[p][q] = inv;

        std::mem::swap(&mut self.basis[p], &mut self.non_basis[q]);
    }

    fn run_simplex(&mut self, obj_row: usize) -> bool {
        loop {
            let mut q = None;
            for j in 0..self.n {
                if self.t[obj_row][j] < -EPS {
                    if q.is_none() || self.non_basis[j] < self.non_basis[q.unwrap()] {
                        q = Some(j);
                    }
                }
            }

            let q = match q {
                Some(col) => col,
                None => return true, // Optimal
            };

            let mut p = None;
            let mut min_ratio = INF;

            for i in 0..self.m {
                if self.t[i][q] > EPS {
                    let ratio = self.t[i][self.n] / self.t[i][q];
                    if ratio < min_ratio - EPS
                        || ((ratio - min_ratio).abs() <= EPS
                            && (p.is_none() || self.basis[i] < self.basis[p.unwrap()]))
                    {
                        min_ratio = ratio;
                        p = Some(i);
                    }
                }
            }

            let p = match p {
                Some(row) => row,
                None => return false, // Unbounded
            };

            self.pivot(p, q);
        }
    }

    pub fn solve(&mut self) -> SimplexResult {
        let mut min_b_row = None;
        let mut min_b = -EPS;
        for i in 0..self.m {
            if self.t[i][self.n] < min_b {
                min_b = self.t[i][self.n];
                min_b_row = Some(i);
            }
        }

        // Phase I (if any b_i < 0)
        if let Some(p) = min_b_row {
            for j in 0..self.n {
                self.t[self.m + 1][j] = 0.0;
            }
            self.t[self.m + 1][self.n] = 0.0;

            for j in 0..=self.n {
                self.t[self.m + 1][j] = -self.t[p][j];
            }

            let mut q = 0;
            for j in 1..self.n {
                if self.t[p][j] < self.t[p][q] {
                    q = j;
                }
            }
            self.pivot(p, q);

            if !self.run_simplex(self.m + 1) || self.t[self.m + 1][self.n] < -EPS {
                return SimplexResult::Infeasible;
            }
        }

        // Phase II
        if !self.run_simplex(self.m) {
            return SimplexResult::Unbounded;
        }

        let mut x = vec![0.0; self.n];
        for i in 0..self.m {
            if self.basis[i] < self.n {
                x[self.basis[i]] = self.t[i][self.n];
            }
        }

        SimplexResult::Optimal(self.t[self.m][self.n], x)
    }
}
```
