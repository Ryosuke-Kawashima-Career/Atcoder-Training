use proconio::{
    input,
    marker::{Chars, Usize1},
};

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

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        ab: [(Usize1, Usize1); m],
    }

    let mut fact = vec![1u64; n + 1];
    let mut inv_fact = vec![1u64; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * i as u64) % MOD;
    }
    inv_fact[n] = inv(fact[n]);
    for i in (1..n).rev() {
        inv_fact[i] = (inv_fact[i + 1] * (i + 1) as u64) % MOD;
    }

    let mut dsu = Dsu::new(n);
    for &(u, v) in &ab {
        dsu.union(u, v);
    }

    let mut comp_nodes = vec![vec![]; n];
    for i in 0..n {
        let root = dsu.find(i);
        comp_nodes[root].push(i);
    }

    let mut total_ways = 1u64;
    let mut has_duplicate = false;
    let mut num_non_trivial = 0;

    for root in 0..n {
        if comp_nodes[root].is_empty() {
            continue;
        }
        let nodes = &comp_nodes[root];
        let sz = nodes.len();
        if sz >= 2 {
            num_non_trivial += 1;
        }

        let mut counts = [0usize; 26];
        for &u in nodes {
            let idx = (s[u] as u8 - b'a') as usize;
            counts[idx] += 1;
        }

        let mut comp_ways = fact[sz];
        for &cnt in &counts {
            if cnt >= 2 {
                has_duplicate = true;
            }
            if cnt > 1 {
                comp_ways = (comp_ways * inv_fact[cnt]) % MOD;
            }
        }

        total_ways = (total_ways * comp_ways) % MOD;
    }

    if !has_duplicate && num_non_trivial >= 1 {
        total_ways = (total_ways * inv(2)) % MOD;
    }

    println!("{}", total_ways);
}
