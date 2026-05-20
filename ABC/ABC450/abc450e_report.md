# ABC450 E: Fibonacci of Strings

## Problem statement

Given $Q$ queries, each consisting of $L, R$, and a character $c$, find the number of occurrences of $c$ in the substring of $S_{10^{18}}$ from the $L$-th to the $R$-th character.

## Model Answer

```rust
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        y: Chars,
        q: usize,
        queries: [(u64, u64, char); q],
    }

    // frequencies for whole strings: freq[m][c]
    let mut freq = vec![vec![0u64; 26], vec![0u64; 26], vec![0u64; 26]];
    // lengths of strings: cur_len[m]
    let mut cur_len = vec![0u64, x.len() as u64, y.len() as u64];

    for i in 0..x.len() {
        let c = (x[i] as u8 - b'a') as usize;
        freq[1][c] += 1;
    }
    for i in 0..y.len() {
        let c = (y[i] as u8 - b'a') as usize;
        freq[2][c] += 1;
    }

    // Precalculate Fibonacci strings up to a length safely > 10^18
    // m is the index of the Fibonacci string, cur_len[m] is the length of the m-th Fibonacci string
    let mut m = 2;
    while cur_len[m] < 1_000_000_000_000_000_000 {
        m += 1;
        let l = cur_len[m - 1] + cur_len[m - 2];
        cur_len.push(l);
        
        let mut f = vec![0u64; 26];
        for c in 0..26 {
            f[c] = freq[m - 1][c] + freq[m - 2][c];
        }
        freq.push(f);
    }

    // Prefix sums for base strings X and Y to answer base-case queries in O(1)
    let x_len = x.len();
    let mut pref_x = vec![vec![0u64; 26]; x_len + 1];
    for i in 0..x_len {
        for c in 0..26 {
            pref_x[i + 1][c] = pref_x[i][c];
        }
        pref_x[i + 1][(x[i] as u8 - b'a') as usize] += 1;
    }

    let y_len = y.len();
    let mut pref_y = vec![vec![0u64; 26]; y_len + 1];
    for i in 0..y_len {
        for c in 0..26 {
            pref_y[i + 1][c] = pref_y[i][c];
        }
        pref_y[i + 1][(y[i] as u8 - b'a') as usize] += 1;
    }

    // Answer queries (prefix matching math: R - (L - 1))
    for (l, r, c_char) in queries {
        let c_idx = (c_char as u8 - b'a') as usize;
        let ans_r = count(m, r, c_idx, &cur_len, &freq, &pref_x, &pref_y);
        let ans_l = count(m, l - 1, c_idx, &cur_len, &freq, &pref_x, &pref_y);
        println!("{}", ans_r - ans_l);
    }
}

// Recursive function to find occurrences of `c` within the first `k` characters of S_m
fn count(
    m: usize, 
    k: u64, 
    c: usize, 
    cur_len: &[u64], 
    freq: &[Vec<u64>], 
    pref_x: &[Vec<u64>], 
    pref_y: &[Vec<u64>]
) -> u64 {
    if k == 0 {
        return 0; // Base case: 0 characters taken
    }
    if m == 1 {
        return pref_x[k as usize][c];
    }
    if m == 2 {
        return pref_y[k as usize][c];
    }
    if k == cur_len[m] {
        return freq[m][c]; // Hit a perfect block
    }

    // S_m is sequentially composed of S_{m-1} heavily skewing left, followed by S_{m-2}
    if k <= cur_len[m - 1] {
        // The prefix perfectly fits inside the left half (S_{m-1})
        return count(m - 1, k, c, cur_len, freq, pref_x, pref_y);
    } else {
        // The prefix covers all of S_{m-1} and spills exactly into S_{m-2}
        let spill_over = k - cur_len[m - 1];
        return freq[m - 1][c] + count(m - 2, spill_over, c, cur_len, freq, pref_x, pref_y);
    }
}
```

---

### Walk Through with a Step by Step Explanation

1. **Understand Prefix Properties of the Formula**
The problem dynamically defines the formula: $S_i = S_{i-1} + S_{i-2}$ (strict concatenation).
Because of this rule, mapping shows that $S_3 = (S_2)S_1$. Then $S_4 = (S_3)S_2$. Thus $S_{k}$ ALWAYS houses $S_{k-1}$ sequentially across its precise left boundary prefix for all $k \ge 3$.
This tells us that the theoretical "infinite-length sequence" $S_{10^{18}}$ just behaves absolutely identical to $S_M$, as long as we find an $M$ where $Length(S_M) \ge 10^{18}$.

2. **Pre-Calculate the Fibonacci Properties**
We launch a relatively invisible `while` loop that builds the dimensions and character frequency arrays for blocks up until the evaluated string breaches length $10^{18}$.
Because it grows identical to a Fibonacci sequence ($F_n$), length $10^{18}$ is breached around index $M \approx 87$. This comfortably tells us all strings can be deconstructed into a maximum of $\approx 87$ massive blocks! It fits well within memory and guarantees absolutely no integer overflow boundaries inside `u64`.

3. **Prefix Computations & Range Query logic**
Before diving into recursion, we create standard constant-time `prefix_sums` matching arrays for variables $S_1$ and $S_2$.
Then for any $L$ to $R$ bracket query thrown at us, we translate it via the classic trick: `Count(R) - Count(L - 1)`.

4. **Recursive Dissection Tree [count(m, k)]**
Finding $K$ characters inside $S_M$:

- If $K$ exactly equals length $S_M$, simply hit the cache and return the precalculated array sum `freq[m][c]`.
- If $K \le S_{M-1}$, drop down the left recursion tree branch directly into `S_{M-1}`.
- If $K > S_{M-1}$, add the entire pre-crunched characters from the left branch, and spill over checking the partial missing block spanning into the right dimension tree `$S_{M-2}$`.

---

### How to come up with the solution

This structure is widely universally grouped under **Fibonacci Word Recursion** or **String Exponentiation**.

Whenever a problem features:

- Abstract rules mathematically fusing String $A$ and String $B$ infinitely.
- Queries demanding arbitrary Indexing or Occurrence Counts into massive bounds $\approx 10^{18}$
- Time limits strongly restricting naive Generation or Index sweeps.

You immediately know that standard iterations are out. The solution is almost always to **Tree Map the properties block-by-block**. The fastest mechanism to solve recursive mathematical string limits is to treat the lengths like a **Segment Tree**. When a queried range is "swallowed" fully by a left-half segment string definition (`k <= left_half`), you tunnel down the structure blindly evaluating only the left. And when the query partially eclipses both parts (`k > left_half`), securely consume the entirety of the left bracket's cached frequency data in $O(1)$, subtract the offset, and query uniquely straight down the right side!
