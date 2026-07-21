use proconio::input;
// Chokudai run j
// Reduce the number of parameters
struct Fenwick {
    data: Vec<i64>,
}
impl Fenwick {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }
    fn new(n: usize) -> Self {
        let data: Vec<i64> = vec![0; n + 1];
        Self { data }
    }
    fn update(&mut self, mut x: usize, val: i64) {
        while x < self.data.len() {
            self.data[x] += val;
            x += Self::lsb(x);
        }
    }
    fn query(&self, mut x: usize) -> i64 {
        let mut sum = 0;
        while x > 0 {
            sum += self.data[x];
            x -= Self::lsb(x);
        }
        sum
    }
}
fn main() {
    input! {n: usize, a: [usize; n]}
    let mut fenwick = Fenwick::new(n);
    let mut inversed: i64 = 0;
    for i in 0..n {
        let cur_val: i64 = fenwick.query(n) - fenwick.query(a[i]);
        inversed += cur_val;
        fenwick.update(a[i], 1);
    }
    println!("{}", inversed);
}
