use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

const MOD: u64 = 998244353;

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

fn ncr(n: u64, r: u64, fact_inv: &[u64]) -> u64 {
    if r > n {
        return 0;
    }
    if r == 0 {
        return 1;
    }
    let mut num = 1u64;
    for i in 0..r {
        let term = (n - i) % MOD;
        num = (num * term) % MOD;
    }
    let den_inv = fact_inv[r as usize];
    (num * den_inv) % MOD
}

fn main() {
    input! {
        n: usize,
        parents: [Usize1; n - 1], // Read exactly n - 1 elements
        candies: [u64; n],
        to_fetch: [u64; n],
    }

    // Build children relationships
    let mut children = vec![vec![]; n];
    for i in 1..n {
        let p = parents[i - 1];
        children[p].push(i);
    }

    // BFS to get topological order
    let mut order = Vec::with_capacity(n);
    let mut queue = VecDeque::new();
    queue.push_back(0);
    order.push(0);

    while let Some(u) = queue.pop_front() {
        for &v in &children[u] {
            queue.push_back(v);
            order.push(v);
        }
    }
    order.reverse(); // Post-order: leaves first, root last

    // Precompute factorials and their inverse factorials
    let max_d = 1_000_005;
    let mut fact = vec![1u64; max_d];
    for i in 1..max_d {
        fact[i] = (fact[i - 1] * i as u64) % MOD;
    }
    let mut fact_inv = vec![1u64; max_d];
    fact_inv[max_d - 1] = power(fact[max_d - 1], MOD - 2);
    for i in (1..max_d - 1).rev() {
        fact_inv[i] = (fact_inv[i + 1] * (i + 1) as u64) % MOD;
    }

    let mut remaining = vec![0u64; n];
    let mut ans = 1u64;

    for &u in &order {
        let mut avail = candies[u];
        for &v in &children[u] {
            avail += remaining[v];
        }

        let r = to_fetch[u];
        if avail < r {
            println!("0");
            return;
        }

        let ways = ncr(avail, r, &fact_inv);
        ans = (ans * ways) % MOD;

        remaining[u] = avail - r;
    }

    println!("{}", ans);
}
