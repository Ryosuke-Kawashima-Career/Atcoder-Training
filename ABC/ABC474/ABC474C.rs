use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [usize; n],
        aq: [usize; q],
    }

    if n == 1 {
        println!("{}", p[0]);
        return;
    }

    // Doubly Linked List represented by arrays of indices (1-indexed values)
    // 0 represents null / sentinel
    let mut prev = vec![0; n + 1];
    let mut next = vec![0; n + 1];

    let mut head = p[0];
    let mut tail = p[n - 1];

    for i in 0..n {
        let v = p[i];
        if i > 0 {
            prev[v] = p[i - 1];
        }
        if i + 1 < n {
            next[v] = p[i + 1];
        }
    }

    for &x in &aq {
        // If the element is already at the tail, do nothing
        if x == tail {
            continue;
        }

        // Remove x from its current position
        if x == head {
            head = next[x];
            prev[head] = 0;
        } else {
            let p_node = prev[x];
            let n_node = next[x];
            next[p_node] = n_node;
            prev[n_node] = p_node;
        }

        // Append x to the tail
        next[tail] = x;
        prev[x] = tail;
        next[x] = 0;
        tail = x;
    }

    // Traverse the list from head to tail to collect the result
    let mut result = Vec::with_capacity(n);
    let mut curr = head;
    while curr != 0 {
        result.push(curr);
        curr = next[curr];
    }

    println!("{}", result.iter().join(" "));
}
