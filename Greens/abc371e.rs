use proconio::input;
// ABC371E
// Q. Σ Σ(i=j<n) f(i, j) such that f(i, j) = the number of kinds of integers in {a_i, a_{i+1}, ..., a_j}.
// A. Segment Tree
fn main() {
    input! {n: usize, a: [i64; n]}
}

#[derive(Clone, Copy)]
struct Monoid {
    x: i64,
}

impl Monoid {
    fn new(x: i64) -> Self {
        Monoid { x }
    }
    /* Monoid has an associative operation and identity element. */
    fn op(&self, b: Self) -> Self {
        Monoid { x: self.x.max(b.x) }
    }
    fn e() -> Self {
        Monoid { x: i64::MIN }
    }
}

struct SegmentTree {
    /* node: 1-indexed leaf: 0-indexed */
    data: Vec<Monoid>,
    n: usize, // number of leaves (power of two)
}

impl SegmentTree {
    #[inline]
    fn next_power_of_two(n: usize) -> usize {
        let mut pow2: usize = 1;
        while pow2 < n {
            pow2 <<= 1;
        }
        pow2
    }

    fn new(n: usize) -> Self {
        let n_pow2: usize = Self::next_power_of_two(n);
        /* total nodes for 1-indexed tree: 2 ^ (ceil(log2(n)) + 1) */
        let data: Vec<Monoid> = vec![Monoid::e(); 2 * n_pow2];
        SegmentTree { data, n: n_pow2 }
    }

    fn build(init: &Vec<i64>) -> Self {
        let n: usize = init.len();
        let n_pow2: usize = Self::next_power_of_two(n);
        /* total nodes for 1-indexed tree: 2 ^ (ceil(log2(n)) + 1) */
        let mut data: Vec<Monoid> = vec![Monoid::e(); 2 * n_pow2];
        for i in 0..n {
            data[i + n_pow2] = Monoid::new(init[i]);
        }
        for i in (1..n_pow2).rev() {
            /* op = | Monoid, Monoid | -> Monoid */
            data[i] = data[2 * i].op(data[2 * i + 1]);
        }
        SegmentTree { data, n: n_pow2 }
    }

    fn update(&mut self, idx0: usize, val: Monoid) {
        /* leaf index in tree: 1-indexed,but external interface 0-indexed */
        let idx: usize = idx0 + self.n;
        self.data[idx] = val;
        let mut cur = idx;
        while cur > 1 {
            cur /= 2;
            self.data[cur] = self.data[2 * cur].op(self.data[2 * cur + 1]);
        }
    }

    fn _query(
        &self,
        l: usize,
        r: usize,
        cur_node: usize,
        cur_left: usize,
        cur_right: usize,
    ) -> Monoid {
        /* data[cur_node] corresponds to [cur_left, cur_right) */
        if r <= cur_left || cur_right <= l {
            return Monoid::e();
        } else if l <= cur_left && cur_right <= r {
            return self.data[cur_node];
        } else {
            let mid: usize = (cur_left + cur_right) / 2;
            let left_val: Monoid = self._query(l, r, 2 * cur_node, cur_left, mid);
            let right_val: Monoid = self._query(l, r, 2 * cur_node + 1, mid, cur_right);
            return left_val.op(right_val);
        }
    }

    fn query(&self, l0: usize, r0: usize) -> Monoid {
        /* l..r: 0-indexed [l, r) */
        /* root node = 1, covers [0, n) */
        self._query(l0, r0, 1, 0, self.n)
    }
}
