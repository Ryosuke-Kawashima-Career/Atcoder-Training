use proconio::{input, marker::Usize1};
use std::io::{self, BufWriter, Write};

#[derive(Clone, Copy, Debug)]
struct Node {
    min_val: usize,
    min_pos: usize,
    max_val: usize,
    max_pos: usize,
}

const INF: usize = usize::MAX;

const IDENTITY: Node = Node {
    min_val: INF,
    min_pos: INF,
    max_val: 0,
    max_pos: INF,
};

fn merge(left: Node, right: Node) -> Node {
    let (min_val, min_pos) = if left.min_val <= right.min_val {
        (left.min_val, left.min_pos)
    } else {
        (right.min_val, right.min_pos)
    };

    let (max_val, max_pos) = if left.max_val >= right.max_val {
        (left.max_val, left.max_pos)
    } else {
        (right.max_val, right.max_pos)
    };

    Node {
        min_val,
        min_pos,
        max_val,
        max_pos,
    }
}

// Iterative (non-recursive) Segment Tree
struct SegmentTree {
    size: usize,
    tree: Vec<Node>,
}

impl SegmentTree {
    fn new(n: usize, p: &[usize]) -> Self {
        let size = n.next_power_of_two();
        let mut tree = vec![IDENTITY; 2 * size];

        for i in 0..n {
            tree[size + i] = Node {
                min_val: p[i],
                min_pos: i,
                max_val: p[i],
                max_pos: i,
            };
        }

        for i in (1..size).rev() {
            tree[i] = merge(tree[2 * i], tree[2 * i + 1]);
        }

        Self { size, tree }
    }

    fn update(&mut self, idx: usize, val: usize) {
        let mut i = idx + self.size;
        self.tree[i] = Node {
            min_val: val,
            min_pos: idx,
            max_val: val,
            max_pos: idx,
        };
        i >>= 1;
        while i > 0 {
            self.tree[i] = merge(self.tree[2 * i], self.tree[2 * i + 1]);
            i >>= 1;
        }
    }

    // Query on half-open range [l, r)
    fn query(&self, mut l: usize, mut r: usize) -> Node {
        let mut res_l = IDENTITY;
        let mut res_r = IDENTITY;

        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 == 1 {
                res_l = merge(res_l, self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = merge(self.tree[r], res_r);
            }
            l >>= 1;
            r >>= 1;
        }

        merge(res_l, res_r)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [usize; n],
        lr: [(Usize1, usize); m], // (L_i - 1, R_i) for 0-indexed half-open range [l, r)
    }

    let mut seg_tree = SegmentTree::new(n, &p);

    for &(l, r) in &lr {
        let node = seg_tree.query(l, r);
        let pos_min = node.min_pos;
        let val_min = node.min_val;
        let pos_max = node.max_pos;
        let val_max = node.max_val;

        // Swap positions in array p
        p[pos_min] = val_max;
        p[pos_max] = val_min;

        // Update the segment tree
        seg_tree.update(pos_min, val_max);
        seg_tree.update(pos_max, val_min);
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for (i, &val) in p.iter().enumerate() {
        if i > 0 {
            write!(out, " ").unwrap();
        }
        write!(out, "{}", val).unwrap();
    }
    writeln!(out).unwrap();
}
