use proconio::input;
const MOD: i64 = 998244353;
fn main() {
    input!{n: usize, k: usize, a: [i64; n]}
    let mut fact: Vec<i64> = vec![0; n+1];
    let mut inv_fact: Vec<i64> = vec![0; n+1];
    for num in 1..=n {
        fact[num] = num * fact[num - 1];
        fact[num] %= MOD;
    }
    for num in (0..n).rev() {
        inv_fact[num] = inv_fact[num + 1] * (num + 1);
        inv_fact[num] %= MOD; 
    }
    let mut n_Cr = |n: usize, r: usize| -> i64 {
        if r > n {
            return 0;
        }
        fact[n] * fact_inv[r] % MOD * fact[n-r] % MOD
    };
    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    for i in 0..n {
        sum1 += a[i];
        sum2 += a[i] * a[i];
        sum1 %= MOD;
        sum2 %= MOD;
    }
    let mut ans: i64 = 0;
    if  n >= 2 && k >= 2 {
        ans += n_Cr(n-2, k-2) * (sum1 * sum1 - sum2) % MOD;
        ans %= MOD;
    }
    if  n >= 1 && k >= 2 {
        ans += n_Cr(n-1, k-2) * (sum1 * sum1 - sum2) % MOD;
        ans %= MOD;
    }
}

fn mod_pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    let base_square: i64 = base * base % MOD;
    let result: i64 = if exp % 2 == 0 {
        mod_pow(base_square, exp / 2) % MOD
    } else {
        base * mod_pow(base_square, (exp - 1) / 2) % MOD
    };
    return result;
}
