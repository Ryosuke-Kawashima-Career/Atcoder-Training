use proconio::input;
struct MaxSegmentTree {
    data: Vec<usize>,
    n: usize,
}

impl MaxSegmentTree {
    fn new(n: usize) -> Self {
        Self {
            data: vec![usize::MAX; 4 * n],
            n,
        }
    }
    fn build(&mut self, a: &[usize], v: usize, l: usize, r: usize) {
        if r - l == 1 {
            self.data[v] = a[l];
        } else {
            let m = (l + r) / 2;
            self.build(a, 2 * v, l, m);
            self.build(a, 2 * v + 1, m, r);
            self.data[v] = self.data[2 * v].max(self.data[2 * v + 1]);
        }
    }
    fn query(&self, v: usize, l: usize, r: usize, target: usize) -> usize {
        if r - l == 1 {
            return l;
        } else {
            let m = (l + r) / 2;
            if self.data[2 * v] >= target {
                return self.query(2 * v, l, m, target);
            } else {
                return self.query(2 * v + 1, m, r, target);
            }
        }
    }
    fn swap(&mut self, a: usize, b: usize) {
        let cur_val_a: usize = self.data[self.n + a];
        let cur_val_b: usize = self.data[self.n + b];
        let mut cur_index_a: usize = self.n + a;
        let mut cur_index_b: usize = self.n + b;

        self.data[cur_index_b] = cur_val_a;
        self.data[cur_index_a] = cur_val_b;
        while cur_index_a > 1 {
            self.data[cur_index_a / 2] = self.data[cur_index_a].max(self.data[cur_index_a ^ 1]);
            cur_index_a >>= 1;
        }
        while cur_index_b > 1 {
            self.data[cur_index_b / 2] = self.data[cur_index_b].max(self.data[cur_index_b ^ 1]);
            cur_index_b >>= 1;
        }
    }
}

struct MinSegmentTree {
    data: Vec<usize>,
    n: usize,
}

impl MinSegmentTree {
    fn new(n: usize) -> Self {
        Self {
            data: vec![usize::MAX; 4 * n],
            n,
        }
    }
    fn build(&mut self, a: &[usize], v: usize, l: usize, r: usize) {
        if r - l == 1 {
            self.data[v] = a[l];
        } else {
            let m = (l + r) / 2;
            self.build(a, 2 * v, l, m);
            self.build(a, 2 * v + 1, m, r);
            self.data[v] = self.data[2 * v].min(self.data[2 * v + 1]);
        }
    }
    fn query(&self, v: usize, l: usize, r: usize, target: usize) -> usize {
        if r - l == 1 {
            return l;
        } else {
            let m = (l + r) / 2;
            if self.data[2 * v] <= target {
                return self.query(2 * v, l, m, target);
            } else {
                return self.query(2 * v + 1, m, r, target);
            }
        }
    }
    fn swap(&mut self, a: usize, b: usize) {
        let cur_val_a: usize = self.data[self.n + a];
        let cur_val_b: usize = self.data[self.n + b];
        let mut cur_index_a: usize = self.n + a;
        let mut cur_index_b: usize = self.n + b;

        self.data[cur_index_b] = cur_val_a;
        self.data[cur_index_a] = cur_val_b;
        while cur_index_a > 1 {
            self.data[cur_index_a / 2] = self.data[cur_index_a].min(self.data[cur_index_a ^ 1]);
            cur_index_a >>= 1;
        }
        while cur_index_b > 1 {
            self.data[cur_index_b / 2] = self.data[cur_index_b].min(self.data[cur_index_b ^ 1]);
            cur_index_b >>= 1;
        }
    }
}
fn main() {
    input! {n: usize, m: usize, p: [usize; n], lr: [(usize, usize); m]}
    let mut mst = MinSegmentTree::new(n);
    let mut pst = MaxSegmentTree::new(n);
    pst.build(&p, 1, 0, n);
    mst.build(&p, 1, 0, n);
    let mut ans = 0;
    for (l, r) in lr {
        pst.swap(l - 1, r - 1);
        mst.swap(l - 1, r - 1);
        let target_max_val = pst.data[1];
        let target_min_val = mst.data[1];
    }
}
