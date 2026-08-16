use proconio::input;
const MOD: i64 = 998244353;
fn pow_mod(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    let base_square: i64 = base * base % MOD;
    let result: i64 = if exp % 2 == 0 {
        pow_mod(base_square, exp / 2) % MOD
    } else {
        base * pow_mod(base_square, (exp - 1) / 2) % MOD
    };
    return result % MOD;
}
fn main() {
    input!{n: usize, k: usize, a: [i64; n]}
    let mut fact: Vec<i64> = vec![1; n + 1];
    let mut inv_fact: Vec<i64> = vec![1; n+1];
    for num in 1..=n {
        fact[num] = num as i64 * fact[num - 1] % MOD;
    }
    inv_fact[n] = pow_mod(fact[n], MOD - 2);
    for num in (0..n).rev() {
        inv_fact[num] = inv_fact[num + 1] * (num + 1) as i64 % MOD;
    }
    let n_cr = |n: usize, r: usize| -> i64 {
        if r >= n {
            return 0;
        }
        fact[n] * inv_fact[r] % MOD * inv_fact[n - r] % MOD
    };
    let mut sum: i64 = 0;
    let mut square_sum: i64 = 0;
    for i in 0..n {
        let ai_mod: i64 = a[i] % MOD;
        sum += ai_mod;
        sum %= MOD;
        square_sum += ai_mod * ai_mod;
        square_sum %= MOD;
    }
    let term1 = n_cr(n-1, k-1) * square_sum % MOD;
    let cross_sum = (sum*sum - square_sum) % MOD;
    let term2 = if k >= 2 {
        n_cr(n-2, k-2) * cross_sum % MOD
    } else {
        0
    };
    let ans = (term1 + term2) % MOD;
    println!("{}", ans);
}