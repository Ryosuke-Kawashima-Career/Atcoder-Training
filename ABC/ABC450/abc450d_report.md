# ABC450 D: Min Max Range

## Problem statement

You can add `K` to any element any number of times. Find the minimum possible value of `max(A) - min(A)`.

## Model Answer

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    if n == 1 {
        println!("0");
        return;
    }

    // 1. Find the absolute maximum in the initial state.
    let max_a = *a.iter().max().unwrap();

    // 2. Add K as many times as possible to each element so that it approaches max_a,
    //    without exceeding max_a. 
    let mut b: Vec<i64> = a
        .iter()
        .map(|&x| {
            let diff = max_a - x;
            let count = diff / k;
            x + count * k
        })
        .collect();

    // 3. Sort the normalized array
    b.sort_unstable();

    // 4. Initial best difference is just sweeping between the normalized max and min
    let mut min_diff = b[n - 1] - b[0];

    // 5. Test shifting the smallest elements past max_a
    for i in 0..n - 1 {
        // We simulate adding K to b[0], b[1], ..., b[i].
        // The new maximum inherently becomes b[i] + K (since b[i] is the largest among the shifted ones, 
        // and b[i] + K > max_a).
        let current_max = b[i] + k;
        
        // The new minimum is simply the next element in the sorted original array, b[i + 1]
        let current_min = b[i + 1];

        let diff = current_max - current_min;
        if diff < min_diff {
            min_diff = diff;
        }
    }

    println!("{}", min_diff);
}
```

---

### Step by Step Explanation

1. **Establish a Cap**: Identify the highest starting value in the array (`max_a`). Because we are only allowed to *add* $K$ (never subtract), this `max_a` acts as the lowest possible roof.
2. **Normalize the Array**: Instead of adding $K$ manually in loops, mathematically teleport all numbers to their highest possible value that does not exceed `max_a`. We do this using integer division `count = (max_a - element) / k`, then doing `element += count * k`. Now, every single number is squeezed into the tightest possible band [(max_a - K, max_a]]. Let's assume we collect these squeezed values in an array $B$.
3. **Sort elements**: Sort $B$ in ascending order. The initial maximum diff is natively just $B[N-1] - B[0]$.
4. **Shift the Minimums**: Now, what happens if we decide to grab the smallest element $B[0]$ and push it over the edge by giving it one more $+K$?
   - It becomes $B[0] + K$. Since $B[0] > \max - K$, adding $K$ guarantees $B[0] + K > \max$. It forcibly becomes the new absolute Maximum of the array.
   - The new Minimum of the array smoothly transitions to $B[1]$.
   - The new difference is just [(B[0] + K) - B[1]].
5. **Sweep them all dynamically**: Iterate $i$ sequentially across the array, simulating giving $B[0]$ through $B[i]$ the $+K$ push. The new array maximum for that step is securely $B[i]+K$, and the new minimum drops straight to $B[i+1]$. We compare this against `min_diff` and save the smallest possible outcome.

---

### How to come up with the solution

This algorithm is a classic competitive programming trope known as **Min-Max Range Minimization** on a circle or periodic boundary.

If you visualize $K$ as a circular clock wrapping around linearly, taking the modulo groups numbers together:

- When a problem says "Find the minimum offset of a group after adding Multiples of $K$", it is identical to projecting points onto a circle of circumference $K$.
- To find the "tightest bundle" of dots on this circle, you want to find the largest empty gap between any two adjacent dots and effectively cut the circle there, unwrapping it into a line. The remaining arc length covering the dots is your minimum maximum discrepancy!

When you normalize limits around `max_a` and sequentially track $B[i]+K - B[i+1]$, you are systematically verifying exactly this: finding the largest gap $B[i+1] - B[i]$ between two adjacent numbers and removing it from the $K$ circumference. Algorithmically, plotting states around sorting sweeps is an incredibly powerful $O(N \log N)$ toolkit!
