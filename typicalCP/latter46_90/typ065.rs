use proconio::input;
const MOD: usize = 998244353;
struct Factorial {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
}
impl Factorial {
    fn init(n: usize) -> Self {
        let mut fact: Vec<usize> = vec![1; n + 1];
        let mut inv_fact: Vec<usize> = vec![1; n + 1];
        for i in 2..=n {
            fact[i] = fact[i - 1] * i % MOD;
        }
        inv_fact[n] = Self::power(fact[n], MOD - 2);
        for i in (2..=n).rev() {
            inv_fact[i - 1] = inv_fact[i] * i % MOD;
        }
        Self { fact, inv_fact }
    }
    fn power(base: usize, power: usize) -> usize {
        if power == 0 {
            return 1;
        }
        let base_square: usize = base * base % MOD;
        let result: usize = if power % 2 == 0 {
            Self::power(base_square, power / 2)
        } else {
            base * Self::power(base_square, (power - 1) / 2) % MOD
        };
        return result;
    }
    fn nCr(&self, n: usize, r: usize) -> usize {
        self.fact[n] * self.inv_fact[r] % MOD * self.inv_fact[n - r] % MOD
    }
}
fn main() {
    input! {red: usize, green: usize, blue: usize, k: usize, red_green_limit: usize, green_blue_limit: usize, blue_red_limit: usize}
    let factorial: Factorial = Factorial::init(red + green + blue);
    let mut ans: usize = 0;
    for r in 0..=red {
        for g in 0..k - r {
            if r + g > k {
                continue;
            }
            let b = k - r - g;
            if r + b > red_green_limit || g + b > green_blue_limit || b + r > blue_red_limit {
                continue;
            }
            if red < r || green < g || blue < b {
                continue;
            }
            ans += factorial.nCr(red, r) % MOD * factorial.nCr(green, g) % MOD
                * factorial.nCr(blue, b)
                % MOD;
        }
    }
    println!("{}", ans % MOD);
}
