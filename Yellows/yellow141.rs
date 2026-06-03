use proconio::input;
// DSL_2_F - RMQ and RUQ
// Q. Range Minimum Query and Range Update Query
// A. Segment Tree (with lazy propagation)
#[derive(Debug, Clone)]
struct LazySegmentTree {
    n: usize,
    data: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    const INF: i64 = i64::MAX;
    #[inline]
    fn next_power_of_2(n: usize) -> usize {
        let mut power = 1;
        while power < n {
            power <<= 1;
        }
        return power;
    }
    fn new(n: usize) -> Self {
        let mut size = Self::next_power_of_2(n);
        Self {
            n: size,
            data: vec![Self::INF; 2 * size],
            lazy: vec![Self::INF; 2 * size],
        }
    }
    fn update(&mut self, l: usize, r: usize, x: i64) {
        self._update(l, r, x, 0, 0, self.n);
    }
    fn _update(&mut self, l: usize, r: usize, x: i64, cell: usize, cell_l: usize, cell_r: usize) {
        // update lazy first
        if self.lazy[cell] != Self::INF {
            self.data[cell] = self.data[cell].min(self.lazy[cell]);
            if cell_r - cell_l > 1 {
                self.lazy[2 * cell] = self.lazy[2 * cell].min(self.lazy[cell]);
                self.lazy[2 * cell + 1] = self.lazy[2 * cell + 1].min(self.lazy[cell]);
            }
        }

        // scope check is second
        if cell_r <= l || r <= cell_l {
            return;
        }
        if l <= cell_l && cell_r <= r {
            self.lazy[cell] = x;
        }
        let mid: usize = (cell_l + cell_r) / 2;
        self._update(l, r, x, 2 * cell, cell_l, mid);
        self._update(l, r, x, 2 * cell + 1, mid, cell_r);
        // update data after children are updated
        self.data[cell] = self.data[2 * cell].min(self.data[2 * cell + 1]);
    }
    fn find_min(&mut self, l: usize, r: usize) -> i64 {
        return self._find_min(l, r, 0, 0, self.n);
    }
    fn _find_min(&mut self, l: usize, r: usize, cell: usize, cell_l: usize, cell_r: usize) -> i64 {

    }
}

fn main() {
    input!{n: usize, q: usize}
    let mut lazysegtree = LazySegmentTree::new(n);
    for _ in 0..q {
        input!{query_type: usize}
        if query_type == 0 {
            input!{l: usize, r: usize, x: i64}
            lazysegtree.update(l, r+1, x);
        } else {
            input!{l: usize, r: usize}
            println!("{}", lazysegtree.find_min(l, r+1));
        }
    }
}