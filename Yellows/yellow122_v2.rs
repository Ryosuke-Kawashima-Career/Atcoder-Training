use proconio::{input, marker::Usize1};
struct SqrtDecomposition {
    data: Vec<i64>,
    backets: Vec<i64>,
    backet_size: usize,
}
impl SqrtDecomposition {
    #[inline]
    fn sqrt(n: usize) -> usize {
        let mut num: usize = 0;
        while num * num < n {
            num += 1;
        }
        num
    }
    fn new(data: &Vec<i64>) -> Self {
        let sqrt_n: usize = Self::sqrt(data.len());
        let mut backets: Vec<i64> = vec![0; sqrt_n];
        for i in 0..data.len() {
            let backet: usize = i / sqrt_n;
            backets[backet] += data[i];
        }
        Self {
            data: data.to_vec(),
            backets,
            backet_size: sqrt_n,
        }
    }
    fn range_sum(&self, left: usize, right: usize) -> i64 {
        /* Calculates the number of sum based on 0-indexed [left, right) */
        let left_backet: usize = left / self.backet_size;
        let right_backet: usize = right / self.backet_size;
        let left_internal: usize = left % self.backet_size;
        let right_internal: usize = right % self.backet_size;
        let mut result: i64 = 0;
        for backet in left_backet + 1..right_backet {
            result += self.backets[backet];
        }
        for i in left_internal..self.backet_size {
            if self.backet_size * left_backet + i < self.data.len() {
                result += self.data[self.backet_size * left_backet + i];
            }
        }
        for i in 0..right_internal {
            if self.backet_size * right_backet + i < self.data.len() {
                result += self.data[self.backet_size * right_backet + i];
            }
        }
        result
    }
    fn ranage_update(&mut self, left: usize, right: usize, val: i64) {
        /* Updates the value in the range of 0-indexed [left, right) by setting the value to `val` */
        let left_backet: usize = left / self.backet_size;
        let right_backet: usize = right / self.backet_size;
        let left_internal: usize = left % self.backet_size;
        let right_internal: usize = right % self.backet_size;
        for backet in left_backet + 1..right_backet {
            self.backets[backet] = val;
        }
        for i in left_internal..self.backet_size {
            if self.backet_size * left_backet + i < self.data.len() {
                self.data[self.backet_size * left_backet + i] = val;
            }
        }
        for i in 0..right_internal {
            if self.backet_size * right_backet + i < self.data.len() {
                self.data[self.backet_size * right_backet + i] = val;
            }
        }
    }
}

fn main() {
    input! {n: usize, m: usize, tlr: [(usize, Usize1, Usize1); m]}
    let mut last_cut_time = SqrtDecomposition::new(&vec![0; n]);
    let mut ans: i64 = 0;
    for i in 0..m {
        let (time, left, right) = tlr[i];
        let sum_of_last_cut: i64 = last_cut_time.range_sum(left, right + 1);
        ans += time as i64 * (right + 1 - left) as i64 - sum_of_last_cut;
        last_cut_time.ranage_update(left, right + 1, time as i64);
    }
    println!("{}", ans);
}
