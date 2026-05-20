use proconio::input;
const MOD: u64 = 998244353;
// ABC458E
// Q. Calculate the number of binary strings of length N such that all the differences between adjacent numbers are at most 1.
// A. Twos devide the sequence into some slots.
fn main() {
    input! {x1: usize, x2: usize, x3: usize}

    let max_n = x1 + x2 + x3 + 1;
    let mut fact = vec![1u64; max_n + 1];
    let mut inv = vec![1u64; max_n + 1];

    for i in 1..=max_n {
        fact[i] = (fact[i - 1] * i as u64) % MOD;
    }

    inv[max_n] = power(fact[max_n], MOD - 2);
    for i in (1..max_n).rev() {
        inv[i] = (inv[i + 1] * (i + 1) as u64) % MOD;
    }

    let ncr = |n: usize, r: usize| -> u64 {
        if r > n {
            return 0;
        }
        let num = fact[n];
        let den = (inv[r] * inv[n - r]) % MOD;
        (num * den) % MOD
    };

    let mut ans = 0u64;
    // k is the number of slots occupied by 1s
    for k in 1..=x1 {
        if k > x2 + 1 {
            break;
        }

        let term1 = ncr(x1 - 1, k - 1);
        let term2 = ncr(x2 + 1, k);
        let term3 = ncr(x2 + x3 - k, x3);

        let res = (((term1 * term2) % MOD) * term3) % MOD;
        ans = (ans + res) % MOD;
    }

    println!("{}", ans);
}

fn power(mut base: u64, mut exp: u64) -> u64 {
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
