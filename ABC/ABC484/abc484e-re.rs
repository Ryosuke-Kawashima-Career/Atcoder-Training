use proconio::input;
struct Fenwick {
    data: Vec<i64>,
}
impl Fenwick {
    #[inline]
    fn lsb(n: usize) -> usize {
        n & n.wrapping_neg()
    }
    fn new(n: usize) -> Self {
        Self {
            data: vec![0; n],
        }
    }
    fn add(&mut self, mut idx: usize) {
        while idx <= self.data.len() {
            self.data[idx] += 1;
            idx = Self::lsb(idx);
        }
    }
    fn sum_query(&self, mut right: usize) -> i64 {
        let mut result: i64 = 0;
        while right > 0 {
            result += self.data[right];
            right -= Self::lsb(right);
        }
        return result;
    }
    fn range_sum_query(&self, left: usize, right: usize) -> i64 {
        if left >= right {
            return 0;
        } else {
            return self.sum_query(right) - self.sum_query(left);
        }
    }
}
fn main() {
    input! {n: usize, l: i64, r: i64, a: [i64; n]}
    let mut prefix: Vec<i64> = vec![0; n+1];
    for i in 0..n {
        prefix[i+1] = prefix[i] + a[i];
    }
    // range_sums for the coordinate compression
    let mut range_sums: Vec<i64> = prefix.clone();
    range_sums.sort_unstable();
    range_sums.dedup();

    // Fenwick is for counting the coordinate compression
    let mut fenwick = Fenwick::new(n);
    let mut ans: i64 = 0;
    for &prefix_sum in &prefix {
        let target_left: i64 = prefix_sum - r;
        let target_right: i64 = prefix_sum - l;
        let index_left: usize = range_sums.partition_point(|&range_sum| range_sum < target_left);
        let index_right: usize = range_sums.partition_point(|&range_sum| range_sum <= target_right);
        ans += fenwick.range_sum_query(index_left, index_right);
        let target_index: usize = range_sums.binary_search(&prefix_sum).unwrap();
        fenwick.add(target_index);
    }
    println!("{}", ans);
}
