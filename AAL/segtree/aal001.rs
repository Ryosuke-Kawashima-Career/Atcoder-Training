use proconio::input;
const IDENTITY: usize = usize::MAX;
struct SegementTree {
    data: Vec<usize>,
    size: usize,
}
impl SegementTree {
    fn new(a: &[usize]) -> Self {
        let n: usize = a.len();
        let size: usize = n.next_power_of_two();
        let mut data: Vec<usize> = vec![IDENTITY; size * 2];
        for i in 0..n {
            data[size + i] = a[i];
        }
        for i in (1..size).rev() {
            data[i] = data[i * 2].min(data[i * 2 + 1]);
        }
        Self { data, size }
    }
    fn query(&self, mut l: usize, mut r: usize) -> usize {
        l += self.size;
        r += self.size;
        let mut res: usize = IDENTITY;
        while l < r {
            if l & 1 == 1 {
                res = res.min(self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res = res.min(self.data[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }
    fn query_recursive(&self, mut l: usize, mut r: usize) -> usize {
        // (index, cell_left, cell_right, query_left, query_right)
        return self._query_recursive(1, 0, self.size, l, r);
    }
    fn _query_recursive(
        &self,
        mut index: usize,
        cell_left: usize,
        cell_right: usize,
        left: usize,
        right: usize,
    ) -> usize {
        if cell_right <= left && right <= cell_left {
            return IDENTITY;
        }
        if left <= cell_left && cell_right <= right {
            return self.data[index];
        }
        let cell_mid: usize = (cell_left + cell_right) / 2;
        let left_result: usize = self._query_recursive(index * 2, cell_left, cell_mid, left, right);
        let right_result: usize =
            self._query_recursive(index * 2 + 1, cell_mid, cell_right, left, right);
        return left_result.min(right_result);
    }
    fn update(&mut self, mut i: usize, x: usize) {
        i += self.size;
        self.data[i] = x;
        while i > 1 {
            self.data[i / 2] = self.data[i].min(self.data[i ^ 1]);
            i >>= 1;
        }
    }
}
fn main() {
    input! {n: usize, q: usize, a: [usize; n], query: [(usize, usize, usize); q]}
    let mut segtree = SegementTree::new(&a);
    for &(query_type, l, r) in query.iter() {
        match query_type {
            1 => {
                let (i, x) = (l - 1, r);
                segtree.update(i, x);
            }
            2 => {
                let (l, r) = (l - 1, r - 1);
                let ans: usize = segtree.query(l, r + 1);
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
