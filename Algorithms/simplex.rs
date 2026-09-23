// Simplex Algorithm implementation in Rust for Linear Programming
//
// Problem Formulation:
// Maximize   c^T x
// Subject to A x <= b,  x >= 0
//
// Supports:
// - Two-phase simplex (handles initial infeasible bases with b_i < 0)
// - Bland's rule for tie-breaking to avoid cycling
// - Detection of Unbounded and Infeasible systems

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
    n: usize,               // Number of decision variables
    t: Vec<Vec<f64>>,       // Tableau of dimension (m + 2) x (n + 1)
    basis: Vec<usize>,      // Basic variable index for each row 0..m
    non_basis: Vec<usize>,  // Non-basic variable index for each column 0..n
}

impl Simplex {
    /// Constructs a Simplex solver for:
    /// maximize    c^T x
    /// subject to  A x <= b, x >= 0
    ///
    /// - a: m x n constraint coefficient matrix
    /// - b: m-dimensional right-hand side vector
    /// - c: n-dimensional objective coefficient vector
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

        Self {
            m,
            n,
            t,
            basis,
            non_basis,
        }
    }

    /// Performs a pivot operation at row `p` and column `q`.
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

    /// Runs simplex iterations with respect to the specified objective row `obj_row`.
    fn run_simplex(&mut self, obj_row: usize) -> bool {
        loop {
            // Entering variable (Bland's rule: pick smallest index among improving columns)
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
                None => return true, // Optimal for this objective row
            };

            // Leaving variable (Minimum ratio test with Bland's tie-breaking)
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
                None => return false, // Objective is unbounded
            };

            self.pivot(p, q);
        }
    }

    /// Solves the LP problem and returns the optimal value and solution vector.
    pub fn solve(&mut self) -> SimplexResult {
        // Phase I: check if any b[i] is strictly negative
        let mut min_b_row = None;
        let mut min_b = -EPS;
        for i in 0..self.m {
            if self.t[i][self.n] < min_b {
                min_b = self.t[i][self.n];
                min_b_row = Some(i);
            }
        }

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

        // Phase II: optimize the actual objective function
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_lp() {
        // Maximize   3x1 + 2x2
        // Subject to:
        //   2x1 +  x2 <= 100
        //    x1 +  x2 <= 80
        //    x1       <= 40
        //   x1, x2 >= 0
        let a = vec![
            vec![2.0, 1.0],
            vec![1.0, 1.0],
            vec![1.0, 0.0],
        ];
        let b = vec![100.0, 80.0, 40.0];
        let c = vec![3.0, 2.0];

        let mut simplex = Simplex::new(&a, &b, &c);
        let result = simplex.solve();

        match result {
            SimplexResult::Optimal(val, x) => {
                assert!((val - 180.0).abs() < 1e-6);
                assert!((x[0] - 20.0).abs() < 1e-6);
                assert!((x[1] - 60.0).abs() < 1e-6);
            }
            _ => panic!("Expected optimal solution"),
        }
    }
}
