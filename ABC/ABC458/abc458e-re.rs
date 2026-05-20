use proconio::input;
const MOD: usize = 998244353;
fn main() {
    input! {x1: usize, x2: usize, x3: usize}
    let x_sum: usize = x1 + x2 + x3;
    let mut fact: Vec<usize> = vec![1; x_sum + 1];
    let mut inv: Vec<usize> = vec![1; x_sum + 1];
    for i in 1..=x_sum {
        fact[i] = (fact[i - 1] * i) % MOD;
    }
    for i in 0..=x_sum {
        inv[i] = power(fact[i], MOD - 2);
    }
    let nCr = |n: usize, r: usize| -> usize {
        if r > n {
            return 0;
        }
        let numerator: usize = fact[n];
        let denominator: usize = (inv[r] * inv[n - r]) % MOD;
        return (numerator * denominator) % MOD;
    };

    let mut ans: usize = 0;
    for k in 1..=x2 {
        let mut cur_val: usize = 1;
        cur_val *= nCr(x2 + 1, k);
        cur_val *= nCr(x1 - 1, k - 1);
        cur_val *= nCr(x3 + x2 - k, x2 - k);
        cur_val %= MOD;
        ans = (ans + cur_val) % MOD;
    }
    println!("{}", ans);
}

fn power(base: usize, exp: usize) -> usize {
    let mut power_result: usize = 1;
    if exp == 0 {
        return power_result;
    }

    let base_squared: usize = (base * base) % MOD;
    if exp % 2 == 0 {
        power_result = power(base_squared, exp / 2) % MOD;
    } else {
        power_result = base * power(base_squared, (exp - 1) / 2) % MOD;
    }
    return power_result % MOD;
}
