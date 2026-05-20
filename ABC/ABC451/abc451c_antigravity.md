# ABC451C: Use Min-Heap like a Stack

## Question

There are $Q$ operations performed on a collection of trees. The operations are as follows:

1. Insert a tree of height $h$.
2. Remove all trees with height less than or equal to $h$.

Count the number of trees remaining after each operation.

## Model Answer

```rust
use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        q: usize,
    }
    
    // Min-Heap to store tree heights. Reverse() wraps the values, forcing pop() to yield the smallest first.
    let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    
    for _ in 0..q {
        input! {
            ty: u8,
            h: i64,
        }
        
        if ty == 1 {
            // Priority Queue Insertion: O(log N)
            heap.push(Reverse(h));
        } else if ty == 2 {
            // Remove trees optimally from the bottom up
            while let Some(&Reverse(min_h)) = heap.peek() {
                if min_h <= h {
                    heap.pop(); // Plucks the smallest tree cleanly in O(log N)
                } else {
                    break; // Because the heap is sorted, anything remaining is > h safely
                }
            }
        }
        
        // Output the number of remaining elements currently in the heap
        println!("{}", heap.len());
    }
}
```

---

### Walk Through with a Step by Step Explanation

This algorithm hinges entirely on an **Amortized Time Complexity** shortcut. Since height $H \le 10^9$, making an array tracker for height $H$ would cause a memory limit crash instantly.
When asked to repeatedly "find all things smaller than bounds" while actively inserting randomly, the universally optimized structure is a Priority Queue (specifically a **Min-Heap**).

1. **Setting up the Base Heap**: Because Rust's built in `std::collections::BinaryHeap` organizes internally as a Max-Heap by default, wrapping values inside the `Reverse(h)` generic physically flips its ordering orientation to bubble the *Smallest* values directly to the top peak.
2. **Handling Insertion Queries (`1 h`)**: Simply cast the tree height inside our reversed struct and let the tree filter it downwards (`heap.push(Reverse(h))`).
3. **Handling Deletion Queries (`2 h`)**:  Since checking the top-most item of the heap is $O(1)$ (`heap.peek()`), we look at the absolute shortest tracked tree across the entire massive garden.
   - If that shortest tree happens to be $\le h$, we chop it down and remove it permanently using `heap.pop()`.
   - The heap elegantly bubbles the *next* shortest tree automatically linearly. So, we dynamically loop this!
   - The absolute millisecond that `peek()` reports a tree that is strictly $> h$, we `break` our $O(\log n)$ cycle entirely. Why? Because if the garden's physically shortest tree is taller than the limit $h$, the algorithm absolutely guarantees all the others are functionally safe!
4. **Buffered IO**: Since $Q = 300,000$, standard Rust `println!` macros will individually buffer to `stdout` 300,000 times natively, which adds $500 \text{ms} \sim 800\text{ms}$ of latency out-of-the-gate. Attaching `proconio::#[fastout]` to `main()` merges everything natively and guarantees you avoid a Time Limit Exception (TLE) regardless of what OS judge runs the code!

---

### How to come up with the solution

When modeling simulation-based problems, look out specifically for these combination flags:

- "Huge Limit Parameters ($H$ is $10^9$)": Direct Array loops and maps are strictly off-limits.
- "Delete everything above/below a certain threshold": Checking items sequentially fails miserably from $O(N)$ overhead.

Any sequence of commands combining **Frequent Insertions + Constant "Extreme Filtering" (Max or Min targeting)** strongly points towards utilizing a Tree Structure mathematically:
While a binary tree `BTreeMap` would have worked flawlessly, relying heavily on a `BinaryHeap` offers incredibly low constant-factor calculation speeds (Array-based continuous memory placement), enabling processing of $\frac{Q}{2}$ cascading deletes without missing a computational breath. Mathematically, since every tree element inserted can strictly only be popped out of the garden *exactly once*, the total amortized cost over ALL $3 \times 10^5$ operations combined stays firmly nested inside a tiny guaranteed $O(Q \log Q)$ bracket ceiling.
