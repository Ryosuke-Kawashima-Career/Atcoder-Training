const EPS: f64 = 1e-9;
const INF: f64 = 1e18;

pub enum SimplexResult {
    Optimal(f64, Vec<f64>),
    Infeasible,
    Unbounded,
}

pub struct SimplexMethod {
    // constraints
    m: usize,
    // variables
    n: usize,
    // t: matrix of inequality constraints
    tableau: Vec<Vec<f64>>,
    // basis variables index
    basis: Vec<usize>,
    // non_basis variables index := slack variables
    non_basis: Vec<usize>,
}

impl SimplexMethod {
    // Maximizes cTx subject to Ax <= b, x >= 0
    // Finds the variable of the maximum potential and its largest bottleneck
    fn new(a: Vec<Vec<f64>>, b: Vec<f64>, c: Vec<f64>) -> Self {
        // a: [constraint_num: m][variable_num: n]
        // b: [constraint_num: m]
        // c: [variable_num: n]
        let m: usize = a.len();
        let n: usize = c.len();
        let mut tableau: Vec<Vec<f64>> = vec![vec![0.0; m + n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                tableau[i][j] = a[i][j];
            }
            tableau[i][i + n] = 1.0;
            tableau[i][m + n] = b[i];
        }
        for j in 0..n {
            tableau[m][j] = -c[j];
        }
    }
    fn pivot(&mut self, p: usize, q: usize) {
        /* Calculates pivoting
        Args:
            p: pivot row
            q: pivot column
        pivot is the bottleneck of the maximum potential value
         */
        let pivot: f64 = self.tableau[p][q];
        // Divide pivot row by pivot element
        for j in 0..self.m + self.n + 1 {
            self.tableau[p][j] /= pivot;
        }
        // Eliminate other rows
        for row in 0..self.m + 1 {
            if row != p {
                let factor: f64 = self.tableau[row][q];
                if factor.abs() > EPS {
                    for j in 0..self.m + self.n + 1 {
                        self.tableau[row][j] -= factor * self.tableau[p][j];
                    }
                } else {
                    // set 0
                    self.tableau[row][q] = 0.0;
                }
            }
        }
        // Update basis
        for col in 0..=self.n {
            if col != q {
                self.tableau[p][col] /= pivot;
            }
        }
        self.tableau[p][q] = 1.0;
        // Swap basis variables
        std::mem::swap(&mut self.basis[p], &mut self.non_basis[q]);
    }
    fn run_simplex(&mut self, obj_row: usize) -> bool {
        /* Simplex method
        Args:
            obj_row: objective row index
        Returns:
            true if optimal solution is found, false otherwise
         */
        loop {
            // Find the pivot col as the maximum potential (Note the values are negative)
            let mut q: Option<usize> = None;
            for col in 0..self.n {
                if q.is_none() || self.non_basis[obj_row][q.unwrap()] < self.non_basis[obj_row][col]
                {
                    q = Some(col);
                }
            }
            let max_potential: f64 = self.non_basis[obj_row][q.unwrap()];
            if max_potential < EPS {
                return true;
            }
            let q: usize = match q {
                Some(q) => q,
                None => return true,
            };

            // Find the pivot row as the hardest bottleneck
            let mut p: Option<usize> = None;
            let mut min_ratio: f64 = INF;
            for row in 0..self.m {
                if self.non_basis[row][q] > EPS {
                    // Constants divided by coefficients of pivot column
                    let ratio: f64 = self.tableau[row][self.m + self.n] / self.non_basis[row][q];
                    if ratio < min_ratio {
                        min_ratio = ratio;
                        p = Some(row);
                    }
                }
            }
            let p: usize = match p {
                Some(p) => p,
                None => return SimplexResult::Unbounded,
            };
            self.pivot(p, q);
        }
    }
    pub fn solve(&mut self) -> SimplexResult {
        /* Simplex method algorithm
        1. Initialization: initial basic feasible solution
        2. Pivoting: find entering and leaving variables
        3. Termination: optimal solution found or unbounded
        */
        let mut min_b_row: Option<usize> = None;
        let mut min_b: f64 = -EPS;
        for row in 0..self.m {
            let b: f64 = self.tableau[row][self.n];
            if min_b.is_none() || b < min_b {
                min_b = b;
                min_b_row = Some(row);
            }
        }
        let min_b_row = match min_b_row {
            Some(row) => row,
            None => return SimplexMethod::Infeasible,
        };
        // Two phase simplex method
        // Phase 1: Find initial basic feasible solution
        for j in 0..self.n {
            self.tableau[self.m + 1][j] = 0.0;
        }
        self.tableau[self.m + 1][self.n] = 0.0;
        let mut q: usize = 0;
        // Phase 2: Optimize objective function
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simplex_method() {
        // Example:
        // Maximize    3x1 + 2x2
        // Subject to:
        //    2x1 +  x2 <= 100
        //     x1 +  x2 <= 80
        //     x1       <= 40
        //   x1, x2 >= 0
        let a = vec![vec![2.0, 1.0], vec![1.0, 1.0], vec![1.0, 0.0]];
        let b = vec![100.0, 80.0, 40.0];
        let c = vec![3.0, 2.0];
        let mut solver = SimplexMethod::new(&a, &b, &c);
        match solver.solve() {
            SimplexResult::Optimal(val, x) => {
                assert_eq!(val, 140.0);
                assert_eq!(x, vec![40.0, 20.0]);
            }
            SimplexResult::Infeasible => println!("Infeasible"),
            SimplexResult::Unbounded => println!("Unbounded"),
        }
    }
}
