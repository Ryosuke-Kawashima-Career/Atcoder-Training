use proconio::{input, marker::Usize1};
// ABC476E
// Q. Swap the maximum and minimum numbers of [l..r]. Repeat Q times.
// A. Get the final state of the array
const INF: i64 = 1 << 60;

#[derive(Clone, Copy, Debug)]
struct Node {
    min_pos: usize,
    min_val: i64,
    max_pos: usize,
    max_val: i64,
}

fn merge(node_left: Node, node_right: Node) -> Node {
    /* Updates the min pos, val and max pos, val */
    let (min_pos, min_val) = if node_left.min_val <= node_right.min_val {
        (node_left.min_pos, node_left.min_val)
    } else {
        (node_right.min_pos, node_right.min_val)
    };
    let (max_pos, max_val) = if node_left.max_val >= node_right.max_val {
        (node_left.max_pos, node_left.max_val)
    } else {
        (node_right.max_pos, node_right.max_val)
    };
    Node {
        min_pos,
        min_val,
        max_pos,
        max_val,
    }
}

const IDENTITY: Node = Node {
    min_pos: usize::MAX,
    min_val: INF,
    max_pos: usize::MAX,
    max_val: -INF,
};
struct SegmentTree {
    size: usize,
    // index starts with 1.
    tree: Vec<Node>,
}

impl SegmentTree {
    #[inline]
    fn next_power_of_two(n: usize) -> usize {
        let mut power_of_two: usize = 1;
        while power_of_two < n {
            power_of_two <<= 1;
        }
        power_of_two
    }

    fn new(array: &[i64]) -> Self {
        let n: usize = array.len();
        let size: usize = Self::next_power_of_two(n);
        let mut tree: Vec<Node> = vec![IDENTITY; size * 2];
        for i in 0..n {
            tree[size + i] = Node {
                min_pos: i,
                min_val: array[i],
                max_pos: i,
                max_val: array[i],
            };
        }
        // Initialize the parent nodes from the leaves
        for idx in (1..size).rev() {
            tree[idx] = merge(tree[idx << 1], tree[(idx << 1) | 1]);
        }
        Self { size, tree }
    }

    fn update(&mut self, pos: usize, val: i64) {
        // Bottom-up update
        let mut idx: usize = pos + self.size;
        self.tree[idx] = Node {
            min_pos: pos,
            min_val: val,
            max_pos: pos,
            max_val: val,
        };
        idx >>= 1;
        while idx > 0 {
            self.tree[idx] = merge(self.tree[idx << 1], self.tree[(idx << 1) | 1]);
            idx >>= 1;
        }
    }

    fn query(&mut self, l: usize, r: usize) -> Node {
        // query of half-open range [l, r).
        let mut node_left: Node = IDENTITY;
        let mut node_right: Node = IDENTITY;
        let mut l: usize = l + self.size;
        let mut r: usize = r + self.size;
        while l < r {
            // if the index is odd
            if l & 1 == 1 {
                node_left = merge(node_left, self.tree[l]);
                l += 1;
            }
            // if the index is even
            if r & 1 == 1 {
                r -= 1;
                node_right = merge(self.tree[r], node_right);
            }
            l >>= 1;
            r >>= 1;
        }
        merge(node_left, node_right)
    }
}

fn main() {
    input! {n: usize, m: usize, mut p: [i64; n], lr: [(Usize1, Usize1); m]}
    let mut min_max_val_pos_segment_tree = SegmentTree::new(&p);
    for &(l, r) in lr.iter() {
        let node = min_max_val_pos_segment_tree.query(l, r + 1);
        let min_index = node.min_pos;
        let max_index = node.max_pos;
        min_max_val_pos_segment_tree.update(min_index, node.max_val);
        min_max_val_pos_segment_tree.update(max_index, node.min_val);
        p[min_index] = node.max_val;
        p[max_index] = node.min_val;
    }
    let ans: String = p
        .iter()
        .map(|&val| val.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
