# ABC 451 D: DFS for String concatenation

## Question

Find the $N$-th smallest positive integer that can be formed by concatenating powers of 2.

## Model Answer

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // Pre-calculate all valid "powers of 2" that can be used up to the 10^9 boundary.
    // Also store their positional multipliers so we can append them mathematically.
    let mut p2s = Vec::new();
    let mut p = 1u64;
    while p <= 1_000_000_000 {
        let s = p.to_string();
        let mult = 10u64.pow(s.len() as u32); // Math equivalent of String Concatenation Shift
        p2s.push((p, mult));
        p *= 2;
    }

    // The maximum theoretical combinations generated under 10^9 limits is ~1.4 million.
    // We pre-allocate to avoid any memory reallocation jitters.
    let mut results = Vec::with_capacity(1_500_000);
    
    // Generate all valid string numbers
    dfs(0, &p2s, &mut results);

    // Filter, Unique, and Fetch
    results.sort_unstable();
    results.dedup();

    println!("{}", results[n - 1]);
}

fn dfs(val: u64, p2s: &Vec<(u64, u64)>, results: &mut Vec<u64>) {
    // If we formed a valid positive combination, save it to the master list
    if val > 0 {
        results.push(val);
    }
    
    // Attempt to string-concatenate the current number heavily with ALL available powers of 2
    for &(p, mult) in p2s.iter() {
        // Concatenating "val" with "p" natively
        let nxt = val * mult + p;
        
        // Hard-cap the growth so we don't blow up our memory out of bounds
        if nxt <= 1_000_000_000 {
            dfs(nxt, p2s, results);
        }
    }
}
```

---

### Walk Through with a Step by Step Explanation

In your provided simulation attempt, you were treating "choosing powers of 2" mathematically like regular sums (e.g. `n + 4` or `n + 8`). The problem statement actively highlights that `1` and `16` combined should strictly generate `116` or `161` (through raw String Concatenation), not `17`.

1. **Modeling String Concatenation as Math**: String allocations (like `format!("{}{}", a, b)`) inside millions of deep loops is incredibly slow. We can fake "string concatenation" mathematically in integers! If I want to attach `16` to the end of `1`, I simply count that `16` possesses *two digits*. Therefore, shifting the first number linearly gives me $1 \times 10^2 + 16 = 116$.
2. **Identifying the Universe of Pieces**: The problem guarantees the highest evaluated integer is capped explicitly at $10^9$. Because any numbers generated strictly grow upward, this tells us there are exactly **30 valid Building Blocks** available (the powers of 2 mapping strictly from $2^0$ to $2^{29}$).
3. **The Combinatorics Check (DFS Boundary)**: Can generating *every physical string* overflow our CPU?  
If we trace the DP counting: lengths $L=1$ combinations yield 4 strings. Lengths $L=4$ combinations yield roughly 400. Expanding this math up to the final barrier Length $L=9$ proves the absolute sum of all possible strings theoretically generated perfectly hits $\approx 1,400,000$. This is absolutely tiny for standard Rust binaries.
4. **Brute Force & Filter**: Armed with the constraint bounds, we simply unleash a fully unrestrained recursive `Depth First Search` (DFS). It continually tries to `math_concatenate` all building blocks together repeatedly until the number breaches $10^9$. Once the recursion finishes, we dump all $1.4\text{M}$ candidates into a `Vec`, `sort` them in roughly 50ms, remove overlaps, and effortlessly index precisely `results[N-1]`!

---

### How to come up with the solution

When modeling string or subset generation problems, aggressively scrutinize the **Constraints Boundary**.

- Any problem mentioning limits identically framed like "The answer is at most $10^9$" paired intimately against a slow "String / Grid Concatenation" requirement usually flags that the combinations space isn't actually astronomically large ($10^9$ strings to build doesn't mean $10^9$ possibilities. Elements like `8` stringed together 9 times grows so explosively that you naturally hit a wall lightning fast).
- If your theoretical combination check verifies the max upper bound outputs `< 5,000,000` states total, **abandon sophisticated logic setups totally**. A naive brute force recursive payload coupled instantly onto `.sort().dedup()` natively handles million-sized clusters optimally inside standard 2.0s timing limits!
