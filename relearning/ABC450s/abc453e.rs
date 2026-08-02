use proconio::input;
// abc453e
// Q. Divide players into two groups with the limitations of player numbers of `left and right`
// A. Sweepline algorithm + combinatorics
// The problem asks for the number of ways to partition the N players into two non-empty teams, Team A and Team B,
// subject to certain constraints based on the "enjoyment range" [L_i, R_i] for each player i.
const MOD: u64 = 998244353;

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
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

fn inv(n: u64) -> u64 {
    mod_pow(n, MOD - 2)
}

struct Comb {
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
}

impl Comb {
    fn new(n: usize) -> Self {
        let mut fact = vec![1; n + 1];
        let mut inv_fact = vec![1; n + 1];
        for i in 1..=n {
            fact[i] = (fact[i - 1] * i as u64) % MOD;
        }
        inv_fact[n] = inv(fact[n]);
        for i in (1..n).rev() {
            inv_fact[i] = (inv_fact[i + 1] * (i + 1) as u64) % MOD;
        }
        Comb { fact, inv_fact }
    }

    fn ncr(&self, n: usize, r: usize) -> u64 {
        if r > n {
            0
        } else {
            self.fact[n] * self.inv_fact[r] % MOD * self.inv_fact[n - r] % MOD
        }
    }
}

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let comb = Comb::new(n);
    let mut events = vec![vec![]; n + 2];

    for i in 0..n {
        let (l, r) = lr[i];
        if l >= 1 && l <= n {
            events[l].push(i);
        }
        if r + 1 >= 1 && r + 1 <= n {
            events[r + 1].push(i);
        }
        if n >= r && n - r >= 1 && n - r <= n {
            events[n - r].push(i);
        }
        if n + 1 >= l && n - l + 1 >= 1 && n - l + 1 <= n {
            events[n - l + 1].push(i);
        }
    }

    let mut aok = vec![0; n];
    let mut bok = vec![0; n];
    let mut count = vec![vec![0usize; 2]; 2];

    // Initialize for k = 1
    for i in 0..n {
        let (l, r) = lr[i];
        if l <= 1 && 1 <= r {
            aok[i] = 1;
        }
        if l <= n - 1 && n - 1 <= r {
            bok[i] = 1;
        }
        count[aok[i]][bok[i]] += 1;
    }

    let mut ans = 0;

    for k in 1..n {
        if k > 1 {
            // Process events for k
            for &player in &events[k] {
                count[aok[player]][bok[player]] -= 1;

                let (l, r) = lr[player];
                aok[player] = if l <= k && k <= r { 1 } else { 0 };
                bok[player] = if l <= n - k && n - k <= r { 1 } else { 0 };

                count[aok[player]][bok[player]] += 1;
            }
        }

        let c00 = count[0][0];
        let c10 = count[1][0];
        let c01 = count[0][1];
        let c11 = count[1][1];

        if c00 == 0 && c10 <= k && c01 <= n - k {
            let ways = comb.ncr(c11, k - c10);
            ans = (ans + ways) % MOD;
        }
    }

    println!("{}", ans);
}
