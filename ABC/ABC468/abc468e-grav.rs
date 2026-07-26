use proconio::input;

const MOD: u64 = 998244353;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut inv = vec![0u64; n + 1];
    inv[1] = 1;
    for i in 2..=n {
        inv[i] = (MOD - MOD / i as u64) * inv[MOD as usize % i] % MOD;
    }

    let mut h = vec![0u64; n + 1];
    for i in 1..=n {
        h[i] = (h[i - 1] + inv[i]) % MOD;
    }

    let mut c = vec![0u64; n];
    c[0] = h[n];
    for a_idx in 1..n {
        c[a_idx] = (c[a_idx - 1] + h[n - a_idx] + MOD - h[a_idx]) % MOD;
    }

    let mut ans = 0u64;
    for i in 0..n {
        let val = (a[i] % MOD) * c[i] % MOD;
        ans = (ans + val) % MOD;
    }

    println!("{}", ans);
}
