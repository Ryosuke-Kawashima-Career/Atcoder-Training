use proconio::input;
struct FenwickXor {
    data: Vec<i64>,
}
impl FenwickXor {
    #[inline]
    fn lsb(n: usize) -> usize {
        n & n.wrapping_neg()
    }
    fn new(n: usize) -> Self {
        let mut data: Vec<i64> = vec![0; n + 1];
        Self { data }
    }
    fn query(&self, mut idx: usize) -> i64 {
        let mut result: i64 = 0;
        while idx > 0 {
            result ^= self.data[idx];
            idx -= Self::lsb(idx);
        }
        result
    }
    fn query_range(&self, left: usize, right: usize) -> i64 {
        if left > right {
            return 0;
        }
        let right_value: i64 = self.query(right);
        if l == 0 {
            return right_value;
        } else {
            return right_value ^ self.query(left - 1);
        }
    }
    fn update(&mut self, mut idx: usize, value: i64) {
        while idx < self.data.len() {
            result ^= 1;
            idx += Self::lsb(idx);
        }
    }
}
fn main() {
    input! {n: usize, q: usize}
    let mut shift: usize = 0;
    let mut b: Vec<i64> = vec![0; n + 1];
    let mut is_active: Vec<bool> = vec![false; q + 5];
    let mut bits: Vec<FenwickXor> = (0..20).map(|k| FenwickXor::new(1 << (k + 1))).collect();
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: usize}
            let prev_value: i64 = b[x];
            let next_value: i64 = (shift as i64).max(prev_value) + 1;
            b[x] = next_value;
            if next_value > shift as i64 {
                is_active[prev_value as usize] = false;
                for keta in 0..20 {
                    let size: usize = 1 << (keta + 1);
                    if (prev_value % size) == 1 {
                        // deactivate the prev_value
                        bits[keta].update(prev_value as usize % size, 1);
                    }
                }
            }
            is_active[next_value] = true;
            for keta in 0..20 {
                let size: usize = 1 << (keta + 1);
                // register the next_value
                bits[keta].update(next_value as usize % size, 1);
            }
        } else {
            shift += 1;
            if is_active[shift] {
                is_active[shift] = 0;
                for keta in 0..20 {
                    let size: usize = 1 << (keta + 1);
                    bits[keta].update(shift & size, 1);
                }
            }
        }
        let mut nim: i64 = 0;
        for keta in 0..20 {
            let size: usize = 1 << (keta + 1);
            let half_size: usize = 1 << keta;
            let start: usize = shift + half_size;
            let end: usize = shift + size - 1;
            let count: i64 = if start <= end {
                bits[keta].query_range(start % size, end % size)
            } else {
                bits[keta].query_range(start % size, size - 1)
                    ^ bits[keta].query_range(0, end % size)
            };
            if count == 1 {
                nim |= 1 << keta;
            }
        }
        println!("{}", nim);
    }
}
