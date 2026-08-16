use proconio::input;
const MOD: i64 = 998244353;
fn main() {
    input!{n: usize, k: usize, a: [i64; n]}
    let mut fact: Vec<i64> = vec![1; n+1];
    let mut inv_fact: Vec<i64> = vec![1; n+1];
    for num in 1..=n {
        fact[num] = num * fact[num - 1];
    }
    inv_fact[n] = mod_pow(fact[n], MOD - 2);
    for num in (0..n).rev() {
        inv_fact[num] = (num + 1) * inv_fact[num + 1];
    }
    let nCr = |n: usize, r: usize| -> i64 {
        if r > n {
            return 0;
        }
        fact[n] * inv_fact[r] % MOD * inv_fact[n - r] % MOD
    };

    let s: i64 = a.iter().sum();
    let mut total_combinations: i64 = nCr(n, k);

    for i in 0..n {
        
    }
}

fn mod_pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    let base_square: i64 = base * base % MOD;
    let result: i64 = if exp % 2 == 0 {
        mod_pow(base_square, exp / 2) % MOD;
    } else {
        base * mod_pow(base_square, exp / 2) % MOD;
    };
    return result % MOD;
}
