use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn from_polar(r: f64, theta: f64) -> Self {
        // TODO 1a: Convert polar coordinates (r, theta) to (re, im)
        let re = r * theta.cos();
        let im = r * theta.sin();
        Self { re, im }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // TODO 1b: Multiply two complex numbers (a + bi)(c + di)
        let a: f64 = self.re;
        let b: f64 = self.im;
        let c: f64 = rhs.re;
        let d: f64 = rhs.im;
        // Re vs Im
        Self::new(a * c - b * d, a * d + b * c)
    }
}

pub fn fft(array: &mut [Complex], invert: bool) {
    let n = array.len();
    assert!(n.is_power_of_two(), "Length must be a power of two");

    // =========================================================================
    // TODO 2: Bit-Reversal Permutation
    // Maintain `j` as the bit-reversal of `i`.
    // At each step `i` from 1..n, add `n >> 1` to `j` with reversed carry propagation
    // and swap `array[i]` with `array[j]` whenever `i < j`.
    // =========================================================================
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        // Your bit-reversal carry loop here
        while (j & bit) != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            array.swap(i, j);
        }
    }

    // =========================================================================
    // TODO 3: Iterative Butterfly Operations
    // For each block size `len` = 2, 4, 8, ..., n:
    // 1. Compute `angle = 2.0 * PI / (len as f64) * (if invert { -1.0 } else { 1.0 })`
    // 2. Compute the principal root `wlen = Complex::from_polar(1.0, angle)`
    // 3. For each block `i` in (0..n).step_by(len):
    //    Initialize `w = Complex::new(1.0, 0.0)`
    //    For `k` in 0..(len / 2):
    //      - u = array[i + k]
    //      - v = array[i + k + len / 2] * w
    //      - array[i + k]           = u + v
    //      - array[i + k + len / 2] = u - v
    //      - w = w * wlen
    // =========================================================================
    let mut len = 2;
    while len <= n {
        // Your butterfly code here
        let angle: f64 = 2.0 * PI / (len as f64) * (if invert { -1.0 } else { 1.0 });
        let wlen: f64 = Complex::from_polar(1.0, angle);
        for i in (0..n).step_by(len) {
            let mut w: f64 = Complex::new(1.0, 0.0);
            for k in 0..(len / 2) {
                let u: f64 = array[i + k];
                // The opposite of u
                let v: f64 = array[i + k + len / 2] * w;
                array[i + k] = u + v;
                array[i + k + len / 2] = u - v;
                w *= wlen;
            }
        }
        len <<= 1;
    }

    // =========================================================================
    // TODO 4: IFFT Normalization
    // If `invert` is true, divide both `re` and `im` of every element by `n as f64`.
    // =========================================================================
    if invert {
        // Your normalization loop here
        for i in 0..n {
            array[i].re /= n as f64;
            array[i].im /= n as f64;
        }
    }
}

pub fn multiply_polynomials(a: &[i64], b: &[i64]) -> Vec<i64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let target_len = a.len() + b.len() - 1;
    let n = target_len.next_power_of_two();

    // =========================================================================
    // TODO 5: 3-Step Convolution Pipeline
    // 1. Create `fa` and `fb` of size `n` padded with zeros, copying `a` and `b`.
    // 2. Run forward FFT (`invert = false`) on both `fa` and `fb`.
    // 3. Multiply `fa[i] = fa[i] * fb[i]` pointwise for all `i in 0..n`.
    // 4. Run inverse FFT (`invert = true`) on `fa`.
    // 5. Round `fa[i].re.round() as i64` for `i in 0..target_len` and return.
    // =========================================================================
    let mut fa: Vec<Complex> = vec![Complex::default(); n];
    let mut fb: Vec<Complex> = vec![Complex::default(); n];
    for i in 0..a.len() {
        fa[i] = Complex::new(a[i] as f64, 0.0);
    }
    for i in 0..b.len() {
        fb[i] = Complex::new(b[i] as f64, 0.0);
    }
    fft(&mut fa, invert = false);
    fft(&mut fb, invert = false);
    for i in 0..n {
        fa[i] = fa[i] * fb[i];
    }
    fft(&mut fa, invert = true);
    let mut result: Vec<i64> = vec![0; target_len];
    for i in 0..target_len {
        result[i] = (fa[i].re.round() as i64);
    }
    result
}

fn main() {
    // Quick self-test: (1 + 2x) * (3 + 4x) = 3 + 10x + 8x^2
    let a = vec![1, 2];
    let b = vec![3, 4];
    let c = multiply_polynomials(&a, &b);
    assert_eq!(c, vec![3, 10, 8]);
    println!("Self-test passed! (1 + 2x)(3 + 4x) = {:?}", c);
}
