use proconio::input;

#[derive(Debug, Clone, Copy)]
struct Node {
    min: i64,
}
impl Node {
    const E: Self = Self { min: i64::MAX };
    fn merge(self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Action {
    add: i64,
}
impl Action {
    const E: Self = Self { add: 0 };
    fn new(add: i64) -> Self {
        Self { add }
    }
    // Apply self first, then other. Additions compose by summing.
    fn compose(self, other: Self) -> Self {
        Self::new(self.add + other.add)
    }
    fn apply(self, node: Node) -> Node {
        // Empty/padded segments remain the identity; avoid infinity + x.
        if node.min == i64::MAX {
            return node;
        }
        Node {
            min: node.min + self.add,
        }
    }
}

struct LazySegmentTree {
    data: Vec<Node>,
    lazy: Vec<Action>,
    size: usize,
}
impl LazySegmentTree {
    fn new(a: &[i64]) -> Self {
        let size = a.len().next_power_of_two();
        let mut data = vec![Node::E; 2 * size];
        for (i, &value) in a.iter().enumerate() {
            data[size + i] = Node { min: value };
        }
        for i in (1..size).rev() {
            data[i] = data[2 * i].merge(data[2 * i + 1]);
        }
        Self {
            data,
            lazy: vec![Action::E; 2 * size],
            size,
        }
    }
    fn apply_at(&mut self, idx: usize, action: Action) {
        // Node is current; lazy is work still owed to its children.
        self.data[idx] = action.apply(self.data[idx]);
        if idx < self.size {
            self.lazy[idx] = self.lazy[idx].compose(action);
        }
    }
    fn propagate(&mut self, idx: usize) {
        let action = self.lazy[idx];
        if action == Action::E {
            return;
        }
        self.apply_at(2 * idx, action);
        self.apply_at(2 * idx + 1, action);
        self.lazy[idx] = Action::E;
    }
    // All internal ranges are zero-based and half-open: [l, r).
    fn update(&mut self, l: usize, r: usize, x: i64) {
        self.update_rec(1, 0, self.size, l, r, Action::new(x));
    }
    fn update_rec(
        &mut self,
        idx: usize,
        left: usize,
        right: usize,
        l: usize,
        r: usize,
        action: Action,
    ) {
        if r <= left || right <= l {
            return;
        }
        if l <= left && right <= r {
            self.apply_at(idx, action);
            return;
        }
        self.propagate(idx);
        let mid = (left + right) / 2;
        self.update_rec(2 * idx, left, mid, l, r, action);
        self.update_rec(2 * idx + 1, mid, right, l, r, action);
        self.data[idx] = self.data[2 * idx].merge(self.data[2 * idx + 1]);
    }
    fn query(&mut self, l: usize, r: usize) -> Node {
        self.query_rec(1, 0, self.size, l, r)
    }
    fn query_rec(&mut self, idx: usize, left: usize, right: usize, l: usize, r: usize) -> Node {
        if r <= left || right <= l {
            return Node::E;
        }
        if l <= left && right <= r {
            return self.data[idx];
        }
        self.propagate(idx);
        let mid = (left + right) / 2;
        let a = self.query_rec(2 * idx, left, mid, l, r);
        let b = self.query_rec(2 * idx + 1, mid, right, l, r);
        a.merge(b)
    }
}

fn main() {
    input! {n: usize, q: usize, a: [i64; n]}
    let mut segtree = LazySegmentTree::new(&a);
    for _ in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {l: usize, r: usize, x: i64}
            segtree.update(l - 1, r, x);
        } else {
            input! {l: usize, r: usize}
            println!("{}", segtree.query(l - 1, r).min);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deferred_and_overlapping_updates() {
        let mut tree = LazySegmentTree::new(&[2, 1, 3, 4]);
        tree.update(0, 4, 5);
        assert_eq!(tree.query(1, 3).min, 6);
        tree.update(1, 3, 10);
        assert_eq!(tree.query(0, 4).min, 7);
        assert_eq!(tree.query(1, 3).min, 16);
    }
    #[test]
    fn compare_with_array() {
        for n in 1..=17 {
            let mut a: Vec<i64> = (0..n).map(|i| i as i64).collect();
            let mut tree = LazySegmentTree::new(&a);
            for step in 0..80 {
                let x = (step * 7) % n;
                let y = (step * 11 + 3) % n;
                let (l, r) = (x.min(y), x.max(y) + 1);
                let add = (step % 5) as i64 * 1_000_000_000;
                tree.update(l, r, add);
                for value in &mut a[l..r] {
                    *value += add;
                }
                for l in 0..n {
                    for r in l + 1..=n {
                        assert_eq!(tree.query(l, r).min, *a[l..r].iter().min().unwrap());
                    }
                }
            }
        }
    }
}
