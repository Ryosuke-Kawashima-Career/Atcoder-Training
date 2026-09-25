use proconio::input;
use proconio::marker::Usize1;
#[derive(Clone, Copy)]
struct Node {
    min_val: i64,
    min_index: usize,
    max_val: i64,
    max_index: usize,
}
impl Node {
    fn merge(&self, other: &Node) -> Self {
        let min_val: i64 = self.min_val.min(other.min_val);
        let min_index: usize;
        if self.min_val <= other.min_val {
            min_index = self.min_index;
        } else {
            min_index = other.min_index;
        }
        let max_val: i64 = self.max_val.max(other.max_val);
        let max_index: usize;
        if self.max_val >= other.max_val {
            max_index = self.max_index;
        } else {
            max_index = other.max_index;
        }
        Node {
            min_val,
            min_index,
            max_val,
            max_index,
        }
    }
}
const IDENTITY: Node = Node {
    min_val: i64::MAX,
    min_index: usize::MAX,
    max_val: i64::MIN,
    max_index: usize::MAX,
};

#[derive(Debug)]
struct SegmentTree {
    data: Vec<Node>,
    size: usize,
}
impl SegmentTree {
    fn new(p: &[usize]) -> Self {
        let n: usize = p.len();
        let size: usize = n.next_power_of_two();
        let data: Vec<Node> = vec![IDENTITY; size * 2];
        for i in 0..n {
            data[size + i] = Node {
                min_val: p[i] as i64,
                min_index: i,
                max_val: p[i] as i64,
                max_index: i,
            };
        }
        for i in (1..size).rev() {
            data[i] = data[i * 2].merge(&data[i * 2 + 1]);
        }
        SegmentTree { data, size }
    }
    fn _query_rec(
        &mut self,
        l: usize,
        r: usize,
        cur_cell: usize,
        cell_left: usize,
        cell_right: usize,
    ) -> Node {
        if cell_left >= r || cell_right < l {
            return IDENTITY;
        }
        if l <= cell_left && cell_right <= r {
            return self.data[cur_cell];
        }
        let mid: usize = (cell_left + cell_right) / 2;
        let left_res: Node = self._query_rec(l, r, 2 * cur_cell, cell_left, mid);
        let right_res: Node = self._query_rec(l, r, 2 * cur_cell + 1, mid, cell_right);
        return left_res.merge(&right_res);
    }
    fn query_rec(&mut self, l: usize, r: usize) -> Node {
        let res: Node = self._query_rec(l, r, 0, 0, self.size);
        res
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> Node {
        /* Return the result of querying the range [l, r) */
        let mut res: Node = IDENTITY;
        l += self.size;
        r += self.size;
        while l < r {
            if l % 2 == 1 {
                res = res.merge(&self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res = res.merge(&self.data[r]);
            }
            l /= 1;
            r /= 1;
        }
        res
    }
    fn swap(&mut self, mut i: usize, mut j: usize) {
        /* Swap values */
        i += self.size;
        j += self.size;
        let val_i: i64 = self.data[i].min_val;
        let val_j: i64 = self.data[j].min_val;
        self.data[i].min_val = val_j;
        self.data[j].min_val = val_i;
        let mut res_i: Node = Node {
            min_val: val_j,
            min_index: i,
            max_val: val_j,
            max_index: i,
        };
        let mut res_j: Node = Node {
            min_val: val_i,
            min_index: j,
            max_val: val_i,
            max_index: j,
        };
        i >>= 1;
        j >>= 1;
        while i > 0 || j > 0 {
            if i > 0 {
                self.data[i] = self.data[i * 2].merge(&self.data[i * 2 + 1]);
                i >>= 1;
            }
            if j > 0 {
                self.data[j] = self.data[j * 2].merge(&self.data[j * 2 + 1]);
                j >>= 1;
            }
        }
    }
}
impl SegmentTree {}
fn main() {
    input! {n: usize, m: usize, p: [usize; n], lr: [(Usize1, Usize1); m]}
    let mut segtree = SegmentTree::new(&p);
    for &(l, r) in lr.iter() {
        let node: Node = segtree.query(l, r + 1);
        segtree.swap(node.min_index, node.max_index);
    }
    for i in 0..n {
        print!("{} ", segtree.data[segtree.size + i].min_val);
    }
    println!();
}
