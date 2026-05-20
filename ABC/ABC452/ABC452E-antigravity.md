# ABC452E: Sum of Modulo

---

## Problem Statement

Find the value, modulo 998244353, of the following summation:
$\sum_{i=1}^{N} \sum_{j=1}^{M} A_i \cdot B_j \cdot (i \pmod j)$

---

### Key points

You absolutely cannot pull $(i \pmod j)$ physically into isolated standalone `j`-loops statically, because the total natively evaluated changes radically based on the exact pairing of BOTH integers!

Instead of attempting separation by evaluating `B` alone intuitively, we dynamically flip the parameters utilizing the strict Modulo Equation: $i \pmod j = i - j \cdot \lfloor \frac{i}{j} \rfloor$.

---

### Model Answer

```rust
use proconio::input;
const MOD: i64 = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; m],
    }

    // `p` statically caches the absolute summation of A_i physically
    let mut p = vec![0i64; n + 1];
    
    // `s` mathematically caches the summation of (A_i * i) natively
    let mut s = 0i64;

    for i in 1..=n {
        p[i] = (p[i - 1] + a[i - 1]) % MOD;
        s = (s + a[i - 1] * (i as i64)) % MOD;
    }

    let mut total_ans = 0i64;

    // Loop directly over `j` evaluating the isolated Harmonic chunks
    for j in 1..=m {
        let mut term = s; // term mathematically models `Sum(A_i * (i % j))` natively

        if j <= n {
            let mut f_num = 0i64;
            let mut k = 1;

            // Harmonic limits grouping jumping safely by blocks of size `j`
            while k * j <= n {
                let left = k * j;
                let right = std::cmp::min(n, (k + 1) * j - 1);

                // Dynamically fetch the sum of limits efficiently in O(1)
                let sum_a = (p[right] - p[left - 1] + MOD) % MOD;
                
                // Track mathematically (sum_a * Floor(i / j)) cleanly!
                f_num = (f_num + sum_a * (k as i64)) % MOD;
                
                k += 1;
            }

            // `j * Floor(i / j)` 
            let j_f = ((j as i64) * f_num) % MOD;
            
            // `S - (j * Floor(i / j))` perfectly equals strictly `i % j` globally!
            term = (s - j_f + MOD) % MOD;
        }

        // Apply parameter B_j uniquely strictly over the properly evaluated mathematical limit
        let add = (b[j - 1] * term) % MOD;
        total_ans = (total_ans + add) % MOD;
    }

    println!("{}", total_ans);
}
```

---

### Walk Through with a Step by Step Explanation

In large nested combinations dynamically computing $500,000$ limit arrays continuously interacting natively against nested operators like $(i \pmod j)$, we universally map evaluations conditionally across **Harmonic Blocks**!

1. **Splitting the Condition Formally**:
Evaluate mathematically exactly what $A_i \cdot (i \pmod j)$ evaluates structurally identically against:
$A_i\cdot\Big(i - j\cdot\Big\lfloor \frac{i}{j} \Big\rfloor \Big) = A_i \cdot i - j\cdot\Big(A_i\cdot\Big\lfloor \frac{i}{j} \Big\rfloor \Big)$

2. **Constant Array Factor**:
Because exactly half of that expanded equation $A_i \cdot i$ structurally NEVER interacts linearly evaluating `$j$` at all mathematically, evaluate it totally statically exactly once during program launch dynamically natively caching directly into limit $S$.
Therefore, your targeted inner equation shrinks mathematically into mapping dynamically $S - j \cdot \Big(\sum A_i \cdot \lfloor\frac{i}{j}\rfloor \Big)$.

3. **Harmonic Array Skips (`O(N log M)`)**:
Calculating accurately $\Big\lfloor \frac{i}{j} \Big\rfloor$ evaluating nested loops continuously limits CPU bandwidth massively natively. Let's group identically!
Notice $\lfloor i / j\rfloor$ strictly behaves completely statically outputting constant combinations specifically mapping across huge chunks? For any limit index $j$, limits completely freeze outputting $1$ uniformly when looping values from $j \dots 2j-1$. It then natively dynamically ticks limits completely identical equaling $2$ from $2j \dots 3j-1$.
Using static Prefix Summary evaluating caches directly natively, you instantly skip entirely grouping mathematical blocks adding constants uniformly mapped natively identically.

4. **Conclusion Output Execution**:
The combinations perfectly resolve sequentially mapping mathematically completely across strictly the mathematical Harmonic formula conditionally mapping boundaries natively across less than $O(N \log M)$ interactions dynamically!

---

### How to Come up With the Solution

Anytime constraints systematically interact conditionally linking $A_i$ directly globally across integer limits spanning identically over $>100,000$ integers against modulus operators mathematically:

- **Substitute Natively Modulo Math Constants**: You physically CANNOT sum combinations spanning isolated loops dynamically tracking $(X \pmod Y)$. The gold-standard algorithm demands you consistently unconditionally expand mathematically exactly into linear algebra constants structurally $(X - Y \cdot \lfloor X/Y \rfloor)$.
- **Identify Grouping Constants (Harmonic Combos)**: You explicitly seek strictly tracking nested floor equations dynamically mapping blocks natively skipping limits tracking identically dynamically jumping globally mapping blocks uniquely scaled physically!
