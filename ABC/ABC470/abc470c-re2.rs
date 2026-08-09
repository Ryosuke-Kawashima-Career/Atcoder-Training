use proconio::input;
struct FenwickXor {
    data: Vec<i64>,
}
impl FenwickXor {
    #[inline]
    fn next_power_of_2(n: usize) -> usize {
        let mut power2: usize = 1;
        while power2 < n {
            power2 <<= 1;
        }
        power2
    }
    #[inline]
    fn lsb(n: usize) -> usize {
        n & usize::wrapping_neg(n)
    }
    fn new(n: usize) -> FenwickXor {
        let size: usize = FenwickXor::next_power_of_2(n);
        FenwickXor {
            data: vec![0; size],
        }
    }

    fn update(&mut self, mut index: usize) {
        let cur_value: i64 = self.data[index];
        let next_value: i64 = cur_value ^ (cur_value + 1);
        while index < self.data.len() {
            self.data[index] ^= 1;
            index += Self::lsb(index);
        }
    }

    fn query(&self, mut index: usize) -> i64 {
        let mut result: i64 = 0;
        while index > 0 {
            result ^= self.data[index];
            index -= Self::lsb(index);
        }
        return result;
    }

    fn query_range(&self, left: usize, right: usize) -> i64 {
        if left > right {
            return 0;
        }
        return self.query(right) ^ self.query(left - 1);
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut fenwick = FenwickXor::new(n);
    let mut a: Vec<usize> = vec![0; n];
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: Usize1}
            if a[x] == 0 {
                fenwick.update(x);
            }
            a[x] += 1;
        } else {
            let result = fenwick.query_range(0, n);
            println!("{}", result);
        }
    }
}
