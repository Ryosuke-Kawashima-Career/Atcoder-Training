use proconio::input;
const MOD: i64 = 1_000_000_007;

// Q. Count the number of integers between 1 and K inclusively
// such that the sum of their digits is divisible by D.
fn main() {
    // FIX 1: K can have up to 10000 digits, it must be read as a String.
    input! {k_str: String, d: usize}
    
    let n_digits: usize = k_str.len();
    let digits: Vec<usize> = k_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    
    // dp[digit][modulo][is_less_than_k: 0: Not yet, 1: Yes]
    let mut dp: Vec<Vec<Vec<Modint>>> =
        vec![vec![vec![Modint::new(0); 2]; d]; n_digits + 1];
    dp[0][0][0] = Modint::new(1);
    
    for i in 0..n_digits {
        for j in 0..d {
            for is_less_than_k in 0..2 {
                let cur_digit = digits[i];
                let upper_bound = if is_less_than_k == 1 { 9 } else { cur_digit };
                
                for digit in 0..=upper_bound {
                    let prev_val = dp[i][j][is_less_than_k];
                    
                    // Cleaned up the state transition logic
                    let next_is_less = if is_less_than_k == 1 || digit < cur_digit { 1 } else { 0 };
                    let next_mod = (j + digit) % d;
                    
                    dp[i + 1][next_mod][next_is_less] += prev_val;
                }
            }
        }
    }
    
    // FIX 2 & 3: Include exact matches for K, and subtract the 0 state
    let answer = dp[n_digits][0][1] + dp[n_digits][0][0] - Modint::new(1);
    println!("{}", answer.val);
}

#[derive(Clone, Copy)]
struct Modint {
    // It calculates the operands of addition with modulo correctly.
    val: i64,
}

impl Modint {
    fn new(val: i64) -> Self {
        // FIX 4: Safe modulo for negative numbers in Rust.
        Self { val: (val % MOD + MOD) % MOD }
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
        self.val = (self.val + rhs.val) % MOD;
    }
}

impl std::ops::Sub for Modint {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.val - rhs.val)
    }
}

impl std::ops::SubAssign for Modint {
    fn sub_assign(&mut self, rhs: Self) {
        self.val = (self.val - rhs.val % MOD + MOD) % MOD;
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
        self.val = (self.val * rhs.val) % MOD;
    }
}
