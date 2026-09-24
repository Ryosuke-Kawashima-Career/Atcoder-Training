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
            data: vec![0; n + 1],
        }
    }
    fn query(&self, mut idx: usize) -> i64 {
        let mut sum: i64 = 0;
        while idx > 0 {
            sum += self.data[idx];
            idx -= Self::lsb(idx);
        }
        return sum;
    }
    fn update(&mut self, mut idx: usize, val: i64) {
        let n: usize = self.data.len();
        while idx < n {
            self.data[idx] += val;
            idx += Self::lsb(idx);
        }
    }
    fn swap(&mut self, idx: usize) {
        let target: i64 = self.data[idx];
        let target_next: i64 = self.data[idx + 1];
        self.update(idx, target_next - target);
        self.update(idx + 1, target - target_next);
    }
}
fn main() {
    input! {n: usize, q: usize, a: [i64; n]}
    let mut fenwick: Fenwick = Fenwick::new(n + 1);
    for i in 0..n {
        fenwick.update(i + 1, a[i]);
    }
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: usize}
            fenwick.swap(x);
        } else {
            input! {l: usize, r: usize}
            let ans: i64 = fenwick.query(r) - fenwick.query(l - 1);
            println!("{}", ans);
        }
    }
}
