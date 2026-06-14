use proconio::input;
struct fenwick {
    size: usize,
    bit: Vec<i64>,
}

impl fenwick {
    fn new(size: usize) -> Self {
        fenwick {
            size,
            bit: vec![0; size + 1],
        }
    }
    fn update(&mut self, index: usize, value: i64) {
        let mut index = index + 1;
        while index <= self.size {
            self.bit[index] += value;
            index += index & index.wrapping_neg();
        }
    }
    fn query(&self, index: usize) -> i64 {
        let mut sum = 0;
        let mut index = index + 1;
        while index > 0 {
            sum += self.bit[index];
            index -= index & index.wrapping_neg();
        }
        sum
    }
    fn query_range(&self, l: usize, r: usize) -> i64 {
        if l > r {
            return 0;
        }
        self.query(r) - self.query(l - 1)
    }
}
fn main() {
    input! {n: usize, d: i64, mut st: [(i64, i64); n]}
    st.sort_by_key(|tup| tup.1);
    let mut count: usize = 0;
    let mut bit = fenwick::new(n);
    for &(s, t) in &st {
        let left: usize = s as usize;
        if t < d {
            continue;
        }
        let right: usize = (t - d) as usize;
        let num_overlap = bit.query_range(left, right);
        count += num_overlap;
        bit.update(s as usize, t as usize, 1);
    }
    println!("{}", count);
}
