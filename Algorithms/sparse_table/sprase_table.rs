fn main() {
    let a = vec![7, 2, 3, 0, 5, 10, 3, 12];
    let st = SparseTable::rmq(&a);
    println!("RMQ(1, 6): {}", st.query(1, 6)); // Expected: 0
    println!("RMQ(4, 7): {}", st.query(4, 7)); // Expected: 3

    // Custom operation example: Range Maximum Query
    let st_max = SparseTable::new(&a, |x, y| x.max(y));
    println!("Range Max(1, 6): {}", st_max.query(1, 6)); // Expected: 10
}

pub struct SparseTable<T, F> {
    /* Handles idempotent functions like min, max, gcd, bitwise AND/OR.
     * Precomputation: O(N log N) time and space.
     * Range Query: O(1) time.
     */
    n: usize,
    st: Vec<Vec<T>>,
    op: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SparseTable<T, F> {
    pub fn new(array: &[T], op: F) -> Self {
        let n = array.len();
        if n == 0 {
            return Self {
                n: 0,
                st: Vec::new(),
                op,
            };
        }
        let k = (n.ilog2() + 1) as usize;
        let mut st = vec![vec![array[0]; n]; k];
        st[0].copy_from_slice(array);

        for j in 1..k {
            let len = 1 << (j - 1);
            for i in 0..=(n.saturating_sub(1 << j)) {
                st[j][i] = op(st[j - 1][i], st[j - 1][i + len]);
            }
        }

        Self { n, st, op }
    }

    pub fn query(&self, l: usize, r: usize) -> T {
        assert!(l < r && r <= self.n, "Range [{}, {}) out of bounds for len {}", l, r, self.n);
        let k = (r - l).ilog2() as usize;
        (self.op)(self.st[k][l], self.st[k][r - (1 << k)])
    }
}

impl<T: Copy + Ord> SparseTable<T, fn(T, T) -> T> {
    pub fn rmq(array: &[T]) -> Self {
        Self::new(array, Ord::min)
    }
}
