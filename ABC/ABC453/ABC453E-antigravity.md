# ABC453E: Combination & Fenwick Tree

## Question

You are given N people. Each person i has a range [L_i, R_i].
We want to divide these N people into two teams, Team A and Team B.
For each person i, if they are assigned to Team A, they must be assigned to a position k such that L_i <= k <= R_i.
Similarly, if assigned to Team B, they must be assigned to a position k such that L_i <= k <= R_i.

We want to find the number of ways to assign all N people to Team A or Team B such that:

1. The number of people in Team A is k.
2. The number of people in Team B is N-k.
3. The positions assigned to Team A are all distinct.
4. The positions assigned to Team B are all distinct.

Constraints:
1 <= N <= 2000
1 <= L_i <= R_i <= N

## Model Answer

```rust
use proconio::input;

const MOD: i64 = 998244353;

// Fast Exponentiation seamlessly calculates extremely deep modular inversion constraints 
fn power(mut base: i64, mut exp: i64) -> i64 {
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

fn mod_inverse(n: i64) -> i64 {
    power(n, MOD - 2)
}

struct Factorial {
    fact: Vec<i64>,
    inv: Vec<i64>,
}

impl Factorial {
    fn new(n: usize) -> Self {
        let mut fact = vec![1; n + 1];
        let mut inv = vec![1; n + 1];
        for i in 1..=n {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }
        inv[n] = mod_inverse(fact[n]);
        for i in (1..n).rev() {
            inv[i] = (inv[i + 1] * (i as i64 + 1)) % MOD;
        }
        inv[0] = 1; // Explicit initial bounds setting
        Self { fact, inv }
    }

    fn comb(&self, n: usize, k: usize) -> i64 {
        if k > n { return 0; }
        let num = self.fact[n];
        let den = (self.inv[k] * self.inv[n - k]) % MOD;
        (num * den) % MOD
    }
}

// Employs a 1D Fenwick sequence directly calculating exactly valid target boundaries uniquely natively
struct Fenwick {
    size: usize,
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            size,
            tree: vec![0; size + 1],
        }
    }
    fn add(&mut self, mut index: usize, value: i64) {
        while index <= self.size {
            self.tree[index] += value;
            index += index & index.wrapping_neg();
        }
    }
    fn query(&self, mut index: usize) -> i64 {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & index.wrapping_neg();
        }
        sum
    }
}

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n]
    }

    // `C(k)` evaluates completely independent statically checking limits identically spanning (L <= k <= R)
    let mut diff = vec![0i64; n + 2];
    for &(l, r) in lr.iter() {
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let mut c = vec![0i64; n + 1];
    let mut cur = 0;
    for i in 1..=n {
        cur += diff[i];
        c[i] = cur;
    }

    // Prepare limit bounds natively for Fenwick sequence
    let mut players = lr.clone();
    players.sort_by_key(|&(l, _)| l);

    let mut s_both = vec![0i64; (n / 2) + 1];
    let mut fenwick = Fenwick::new(n + 1);
    let mut idx = 0;

    // Linearly aggregate nested combinations directly uniquely where Both elements span overlaps!
    for m in 1..=(n / 2) {
        while idx < n && players[idx].0 <= m {
            fenwick.add(players[idx].1, 1);
            idx += 1;
        }
        // Identifies exactly how many `R` parameters span greater natively mapped mathematically >= (N - m)
        s_both[m] = fenwick.query(n) - fenwick.query(n - m - 1);
    }

    let fact = Factorial::new(n);
    let mut total_ways = 0i64;

    for k in 1..n {
        let m = std::cmp::min(k, n - k);
        let both = s_both[m];
        
        // S_none tracks mathematically precisely subsets exclusively locked out entirely evaluating limits!
        let none = n as i64 - c[k] - c[n - k] + both;
        if none > 0 {
            continue; // At least one player structurally CANNOT align with Team A nor Team B sizes logically
        }

        // S_a tracks completely players forcibly demanding exclusively mapping to Team A identically
        let s_a = c[k] - both;
        
        // Math Bounds validating target lengths accurately map safely within parameter combinations internally!
        if (k as i64) >= s_a {
            let r = (k as i64) - s_a;
            if r <= both {
                // To fill out Team A, uniquely isolate `r` identical players universally chosen structurally
                total_ways = (total_ways + fact.comb(both as usize, r as usize)) % MOD;
            }
        }
    }

    println!("{}", total_ways);
}
```

---

### Walk Through with a Step by Step Explanation

1. **Calculate Fixed Length Models (Team Limits)**:
Rather than attempting grouping sets uniformly tracking conditionally combinations explicitly, we universally assume Team A sizes identical static constraint vectors spanning linearly $k \in (1 \ldots N-1)$. Thus guaranteeing identically Team B tracks completely functionally exactly $N - k$.

2. **The 3 Venn-Diagram Groups ($S_A, S_B, S_{Both}$)**:
For every specific $k$ combination length modeled unconditionally natively mapping arrays uniquely, each player falls entirely mutually-exclusively uniformly inside exactly one group natively:

- **`S_None`**: Parameters mathematically failing directly explicitly tracking $(L_i \le K \le R_i)$ functionally AND explicitly failing tracking mathematically $(L_i \le N-k \le R_i)$ fundamentally fail globally! If uniquely even ONE player tracks natively mapping identically hitting constraints bounds uniquely unconditionally completely inside $S_{None}$, dynamically mapping Team subsets strictly functionally equating `Ways = 0`!
- **`S_A`, `S_B`**: Limits spanning explicitly conditional limits tracking mapping uniquely strictly matching size identical uniformly. Players conditionally caught explicitly tracking inside $S_A$ inherently uniquely strictly exclusively MUST map seamlessly identifying uniformly Team A functionally unconditionally!
- **`S_both`**: Parameters specifically structurally uniformly overlapping intersecting perfectly explicitly structurally mapping Team A completely alongside aligning completely mappings matching uniquely conditional mapping $Team B$ evenly!
