# ABC454F: Palindrome

## problem Statement

A=(A1 ,A2,…,AN) of length N. It is guaranteed that each element of A is between 0 and M−1, inclusive.
You can perform the following operation on the integer sequence A any number of times, possibly zero: Choose a pair of integers (l,r) satisfying 1≤l≤r≤N, and replace Ai with (Ai+1)modM for each i=l,l+1,…,r.
Find the minimum number of operations required to make A a palindrome.

---

### Model Answer

```rust
use proconio::input;

fn solve(m: usize, a: &[usize]) -> usize {
    let n = a.len();
    let k = n / 2;
    
    // b_i tracks the difference between symmetric pairs
    let mut b = vec![0; k + 2];
    for i in 1..=k {
        let left = a[i - 1]; // 0-indexed mappings
        let right = a[n - i];
        b[i] = (right + m - left) % m;
    }
    
    // d_i represents the difference array of b
    let mut d = vec![0; k + 1];
    let mut sum_d = 0;
    for i in 1..=k+1 {
        d[i - 1] = (b[i] + m - b[i - 1]) % m;
        sum_d += d[i - 1];
    }
    
    // c represents exactly how many shifts of -M we must apply
    let c = sum_d / m;
    d.sort_unstable(); // Sort to find the C largest values
    
    let mut cost = 0;
    for i in 0..(k + 1) {
        // Apply the -M shifts to the C largest elements to minimize the absolute values
        if i >= (k + 1) - c {
            cost += m - d[i];
        } else {
            cost += d[i];
        }
    }
    
    // Since each range operation alters exactly two bounds by +1 and -1, operations = sum(|D|) / 2
    cost / 2
}

fn main() {
    input! { t: usize }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            a: [usize; n]
        }
        let min_ops = solve(m, &a);
        println!("{}", min_ops);
    }
}
```

### Walk-through with a Step by Step Explanation

Your previous code iterated over elements and attempted to modulo update linearly, but adding to elements via a range `[l, r]` modifies elements together. This implies checking overlapping states dynamically logic would be heavily prone to $O(N^2)$ tracking flaws.

Instead of tracking values of `A` in isolated cases, the most optimal way to handle any range problem is breaking it down mathematically via array differences. Let's step exactly how we reach the solution:

1. **Focus on Symmetry Differences:** Instead of trying to construct `A`, we only need $A_i$ and $A_{N-1-i}$ to end up equal. Let's form an array $B$ of size $K = \lfloor N/2 \rfloor$ where $B_i = (A_{N-i} - A_i) \pmod M$. Our goal is strictly to make the $B$ array evaluate to $0 \pmod M$.
2. **Translate The Operation:** If we increment a range inside `A`, how does $B$ interpret that? Adding $+1$ to `A` mapped symmetrically happens exactly to add $+1$ or $-1$ to a *contiguous valid subsegment* entirely isolated within $B$. A range increment on `A` corresponds to precisely exactly ONE range `+1` or `-1` operation bounded on $B$.
3. **Construct Difference Array:** We want to build an abstract operation $E_i \equiv -B_i \pmod M$. Since operations are ranges, the "cost" to build $E$ with range increments/decrements is famously dictated by its difference array, $D_i = E_i - E_{i-1}$, summing precisely to $\frac{1}{2}\sum_{i=1}^{K+1} |D_i|$ moves. (Where $E_0 = E_{K+1} = 0$).
4. **Greedy Assignment:** Since we take absolute signs, $D_i$ wants to either shift towards $d_i$ directly, or its modulo wraparound via pulling $-M$. We sum the raw positive modulo'd array rawly, noticing that $\sum d_i$ must total a perfect multiple of $M$, specifically $C \cdot M$. This informs us exactly that subtracting $-M$ shifts requires us to invoke it precisely $C$ times! Sorting it to use those instances cleanly on the highest $d_i$ peaks will minimize total absolute distances.

### How to Come Up with the Solution

1. **Extract Invariants**: The central element (when $N$ is odd) doesn't possess a mirrored partner. It will always equal itself, meaning operations on it literally don't matter towards the goal. This hints deeply that we should isolate mirrored pairs $A_i \iff A_{N-i}$ to check requirements.
2. **Abstract the Action Space:** Whenever operations apply onto connected ranges on some $A$ where you wish to unify array values, analyzing standard transformations on "Subtractive Array mapping" $D_i = A_i - A_{i-1}$ forces boundary manipulation into $+1$ / $-1$ node charges. Applying difference topologies breaks complex interval weaving rules directly into independently assignable endpoints where cost equates purely to absolute magnitude tracking! In essence, this technique fundamentally avoids generating combinations, turning it into purely math.
