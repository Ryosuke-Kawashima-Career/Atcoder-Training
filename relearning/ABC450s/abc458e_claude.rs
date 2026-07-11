use proconio::input;

const MOD: i64 = 998244353;

struct Comb {
    fact: Vec<i64>,
    fact_inv: Vec<i64>,
}

impl Comb {
    fn new(max_n: usize) -> Self {
        let mut fact = vec![1i64; max_n + 1];
        for i in 1..=max_n {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }
        let mut fact_inv = vec![1i64; max_n + 1];
        fact_inv[max_n] = pow_mod(fact[max_n], MOD - 2);
        for i in (1..=max_n).rev() {
            fact_inv[i - 1] = fact_inv[i] * i as i64 % MOD;
        }
        Self { fact, fact_inv }
    }

    // C(n, r) mod MOD, safe for any n, r (returns 0 when out of range).
    fn ncr(&self, n: i64, r: i64) -> i64 {
        if r < 0 || n < 0 || r > n {
            return 0;
        }
        let (n, r) = (n as usize, r as usize);
        self.fact[n] * self.fact_inv[r] % MOD * self.fact_inv[n - r] % MOD
    }
}

fn pow_mod(mut base: i64, mut exp: i64) -> i64 {
    base %= MOD;
    let mut result: i64 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

fn main() {
    input! { x1: i64, x2: i64, x3: i64 }
    let x_sum = x1 + x2 + x3;
    let comb = Comb::new(x_sum as usize);

    // Lay the X2 copies of "2" out in a row: this creates X2+1 gaps
    // (before the first 2, between each pair, after the last 2).
    // Because 1 and 3 can never sit next to each other, every gap must be
    // entirely empty, entirely a run of 1's, or entirely a run of 3's;
    // gaps separated by at least one "2" never touch, so their contents
    // never interact.
    //
    // For each k = number of gaps reserved for 3's (each gets >= 1 three):
    //   - choose which k of the (X2+1) gaps hold 3's:      C(X2+1, k)
    //   - distribute X3 threes into those k gaps (each>=1): C(X3-1, k-1)
    //   - distribute X1 ones into the remaining (X2+1-k)
    //     gaps, allowing empty gaps:                        C(X1+X2-k, X2-k)
    let mut ans: i64 = 0;
    for k in 1..=(x2 + 1) {
        let choose_gaps = comb.ncr(x2 + 1, k);
        let place_threes = comb.ncr(x3 - 1, k - 1);
        let place_ones = comb.ncr(x1 + x2 - k, x2 - k);
        ans = (ans + choose_gaps * place_threes % MOD * place_ones % MOD) % MOD;
    }
    println!("{}", ans);
}