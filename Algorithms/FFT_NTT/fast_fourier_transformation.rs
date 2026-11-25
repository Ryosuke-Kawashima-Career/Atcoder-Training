use proconio::input;
#[derive(Debug, Copy, Clone, Default)]
struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    // cos(a+b) = cos(a)cos(b) - sin(a)sin(b)
    // sin(a+b) = sin(a)cos(b) + cos(a)sin(b)
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

pub fn fft(array: &mut [Complex], invert: bool) {
    use std::f64::consts::PI;
    /* In-place Cooley-Tukey FFT and IFFT algorithm */
    let n: usize = array.len();
    assert!(n.is_power_of_two(), "Length must be a power of two");

    // Bit-reversal permutation
    let mut j: usize = 0;
    for i in 1..n {
        let mut bit: usize = n >> 1;
        while (j & bit) != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            array.swap(i, j);
        }
    }
    // Iterative Butterfly Operations
    let mut len: usize = 2;
    while len <= n {
        let angle: f64 = 2.0 * PI / (len as f64) * (if invert { -1.0 } else { 1.0 });
        let wlen: Complex = Complex::from_polar(1.0, angle);
        for i in (0..n).step_by(len) {
            let mut w: Complex = Complex::new(1.0, 0.0);
            for k in 0..(len / 2) {
                let u: Complex = array[i + k];
                let v: Complex = array[i + k + len / 2] * w;
                array[i + k] = u + v;
                array[i + k + len / 2] = u - v;
                w = w * wlen;
            }
        }
        len <<= 1;        
    }
    if invert {
        let n_f64: f64 = n as f64;
        for x in array.iter_mut() {
            x.re /= n_f64;
            x.im /= n_f64;
        }
    }
}

pub fn multiply_polynomials(a: &[i64], b: &[i64]) -> Vec<i64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let target_len: usize = a.len() + b.len() - 1;
    let n: usize = target_len.next_power_of_two();
    let mut fa: Vec<Complex> = vec![Complex::default(); n];
    let mut fb: Vec<Complex> = vec![Complex::default(); n];
    for i in 0..a.len() {
        fa[i] = Complex::new(a[i] as f64, 0.0);
    }
    for i in 0..b.len() {
        fb[i] = Complex::new(b[i] as f64, 0.0);
    }
    fft(&mut fa, false);
    fft(&mut fb, false);
    // Point wise multiplication
    for i in 0..n {
        fa[i] = fa[i] * fb[i];
    }
    // Inverse FFT
    fft(&mut fa, true);
    // Float -> Int
    let mut result: Vec<i64> = vec![0; n];
    for i in 0..target_len {
        result[i] = fa[i].re.round() as i64;
    }
    result.truncate(target_len);
    result
}

// AtCoder Typical Contest 001 C - (Fast Fourier Transform)
// Q. Count combinations where Main dish (price i) + Side dish (price j) = k for 1 <= k <= 2N
// Complexity: O(N log N) using Cooley-Tukey FFT
fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }

    // A[i] is the number of main dishes costing i yen (1-indexed, so degree up to N)
    // B[i] is the number of side dishes costing i yen (1-indexed, so degree up to N)
    let mut a = vec![0i64; n + 1];
    let mut b = vec![0i64; n + 1];
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        a[i + 1] = ai;
        b[i + 1] = bi;
    }

    // Convolution: c[k] = sum_{i+j = k} a[i] * b[j]
    let c = multiply_polynomials(&a, &b);

    for k in 1..=2 * n {
        let ans = if k < c.len() { c[k] } else { 0 };
        println!("{}", ans);
    }
}