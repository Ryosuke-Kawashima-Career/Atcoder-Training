use proconio::input;

const MOD: i64 = 998244353;

fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    let mut res = 1;
    base %= MOD;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % MOD;
        }
        base = (base * base) % MOD;
        exp /= 2;
    }
    res
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }

    let mut fact = vec![1; n + 1];
    let mut inv_fact = vec![1; n + 1];

    for i in 1..=n {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
    }
    inv_fact[n] = mod_pow(fact[n], MOD - 2);
    for i in (0..n).rev() {
        inv_fact[i] = (inv_fact[i + 1] * (i + 1) as i64) % MOD;
    }

    let n_cr = |n: usize, r: usize| -> i64 {
        if r > n {
            return 0;
        }
        fact[n] * inv_fact[r] % MOD * inv_fact[n - r] % MOD
    };

    let mut s1: i64 = 0;
    let mut s2: i64 = 0;
    for &x in &a {
        let x_mod = x % MOD;
        s1 = (s1 + x_mod) % MOD;
        s2 = (s2 + x_mod * x_mod) % MOD;
    }

    let term1 = (n_cr(n - 1, k - 1) * s2) % MOD;

    let cross_sum = ((s1 * s1 % MOD - s2) % MOD + MOD) % MOD;
    let term2 = if k >= 2 {
        (n_cr(n - 2, k - 2) * cross_sum) % MOD
    } else {
        0
    };

    let ans = (term1 + term2) % MOD;
    println!("{}", ans);
}
