fn main() {
    let a = vec![7, 2, 3, 0, 5, 10, 3, 12];
    let st = SparseTable::rmq(&a);
    println!("RMQ(1, 6): {}", st.query(1, 6)); // Expected: 0
    println!("RMQ(4, 7): {}", st.query(4, 7)); // Expected: 3

    // Custom operation example: Range Maximum Query
    let st_max = SparseTable::new(&a, |x, y| x.max(y));
    println!("Range Max(1, 6): {}", st_max.query(1, 6)); // Expected: 10
}
struct SparseTable<T, F> {
    st: Vec<Vec<T>>,
    op: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SparseTable<T, F> {
    fn new(array: &[T], op: F) -> Self {
        let n: usize = array.len();
        if n == 0 {
            return Self {
                st: vec![],
                op,
            };
        }
        let k: usize = (n.ilog2() + 1) as usize;
        let mut st: Vec<Vec<T>> = vec![vec![array[0]; n]; k];
        for j in 1..k {
            let length: usize = 2usize.pow((j - 1) as u32);
            for i in 0..=(n.saturating_sub(length)) {
                st[i][j] = op(st[i][j - 1], st[i + length][j - 1])
            }
        }
        Self { st, op }
    }

    fn query(&self, l: usize, r: usize) -> T {
        assert!(l < r && r <= self.n)
        let length: usize = r - l;
        let level: usize = length.ilog2() as usize;
        let power_of_2: usize = 1 << level;
        let result: T = (self.op)(self.st[level][l], self.st[level][r.saturating_sub(power_of_2)]);
        return result;
    }
}

impl<T: Copy + Ord> SparseTable<T, fn(T, T) -> T> {
    fn range_min(array: &[T]) -> Self {
        Self::new(array, Ord::min)
    }
}