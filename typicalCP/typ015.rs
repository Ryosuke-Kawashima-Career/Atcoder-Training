use proconio::input;

const MOD: usize = 1_000_000_007;

fn mod_pow(mut base: usize, mut exp: usize) -> usize {
    let mut res = 1;
    base %= MOD;
    while exp > 0 {
        if exp % 2 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        exp /= 2;
    }
    res
}

fn main() {
    input! {
        n: usize,
    }

    // Step 1: Precompute factorials and inverse factorials in O(N)
    let mut fact = vec![1; n + 1];
    let mut inv_fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i % MOD;
    }
    inv_fact[n] = mod_pow(fact[n], MOD - 2);
    for i in (1..=n).rev() {
        inv_fact[i - 1] = inv_fact[i] * i % MOD;
    }

    // Helper closure to compute nCr in O(1)
    let ncr = |n_val: usize, r_val: usize| -> usize {
        if n_val < r_val {
            return 0;
        }
        fact[n_val] * inv_fact[r_val] % MOD * inv_fact[n_val - r_val] % MOD
    };

    // Step 2: For each k, sum over valid number of chosen balls `a`
    for k in 1..=n {
        let mut ans = 0;
        let mut a = 1;
        while n >= (k - 1) * (a - 1) + a {
            let rem = n - (k - 1) * (a - 1);
            ans = (ans + ncr(rem, a)) % MOD;
            a += 1;
        }
        println!("{}", ans);
    }
}
