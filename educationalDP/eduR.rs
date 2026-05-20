use proconio::input;
const MOD: i64 = 1_000_000_007;
// warshall floyd method
// 行列の計算のk乗を求めることで、iからjへの長さkのパスの数を求める
fn main() {
    input! {n: usize, k: usize, a: [[i64; n]; n]}
    let adjacent_matrix: Matrix = Matrix::new(&a);
    let result_matrix = adjacent_matrix.pow(k);
    let mut total_paths = Modint::new(0);
    for i in 0..n {
        for j in 0..n {
            total_paths += result_matrix.data[i][j];
        }
    }
    println!("{}", total_paths.val);
}

#[derive(Clone)]
struct Matrix {
    data: Vec<Vec<Modint>>,
}

impl Matrix {
    fn new(source: &Vec<Vec<i64>>) -> Self {
        /* The source should be a square matrix. */
        let mut data: Vec<Vec<Modint>> = Vec::new();
        let n: usize = source.len();
        for i in 0..n {
            let mut row: Vec<Modint> = Vec::new();
            for j in 0..n {
                row.push(Modint::new(source[i][j]));
            }
            data.push(row);
        }
        Self { data }
    }

    fn mul(&self, other: &Matrix) -> Self {
        let n: usize = self.data.len();
        let mut result: Matrix = Matrix::new(&vec![vec![0; n]; n]);
        for i in 0..n {
            for k in 0..n {
                for j in 0..n {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    fn pow(&self, exponent: usize) -> Self {
        let n: usize = self.data.len();
        let mut result: Matrix = Matrix::new(&vec![vec![0; n]; n]);
        for i in 0..n {
            result.data[i][i] = Modint::new(1);
        }
        let mut base = self.clone();
        let mut exponent: usize = exponent;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exponent /= 2;
        }
        result
    }
}

#[derive(Clone, Copy)]
struct Modint {
    // It calculates the operands of addition with modulo correctly.
    // std::ops::Add and AddAssign are implemented.
    val: i64,
}

impl Modint {
    fn new(val: i64) -> Self {
        Self { val: val % MOD }
    }
}

impl std::ops::Add for Modint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.val + rhs.val)
    }
}

impl std::ops::AddAssign for Modint {
    fn add_assign(&mut self, rhs: Self) {
        self.val += rhs.val;
        self.val %= MOD;
    }
}

impl std::ops::Mul for Modint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.val * rhs.val)
    }
}

impl std::ops::MulAssign for Modint {
    fn mul_assign(&mut self, rhs: Self) {
        self.val *= rhs.val;
        self.val %= MOD;
    }
}
