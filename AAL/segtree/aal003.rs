use proconio::input;
#[derive(Debug, Clone, Copy)]
struct SegmentTree {
    data: Vec<i64>,
    size: usize,
}
impl SegmentTree {
    fn new(a: &[i64]) -> Self {
        let n: usize = a.len();
        let size: usize = n.next_power_of_two();
        let data: Vec<i64> = vec![0; size * 2];
        for i in 0..n {
            data[size + i] = a[i];
        }
        for i in (1..size).rev() {
            data[i] = data[i * 2] + data[i * 2 + 1];
        }
        Self { data, size }
    }
    fn update(&mut self, mut i: usize, x: i64) {
        i += self.size;
        self.data[i] += x;
        while i > 1 {
            self.data[i / 2] = self.data[i] + self.data[i ^ 1];
            i /= 2;
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> i64 {
        let mut result: i64 = 0;
        l += self.size;
        r += self.size;
        while r - l > 1 {
            if l & 1 == 1 {
                result += self.data[l];
                l += 1;
            }
            if r & 1 == 1 {
                result += self.data[r];
                r -= 1;
            }
            l /= 2;
            r /= 2;
        }
        result
    }
    fn binary_search_left(&mut self, t: i64, mut l: usize) -> usize {}
}
fn main() {
    input! {n: usize, q: usize, a: [i64; n], queries: [(usize, usize, i64); q]}
    let mut segtree = SegmentTree::new(&a);
    for query in 0..q {
        let query_type: usize = queries[query].0;
        if query_type == 1 {
            let (i, x) = (queries[query].1 - 1, queries[query].2);
            segtree.update(i, x);
        } else if query_type == 2 {
            let (l, t) = (queries[query].1 - 1, queries[query].2 - 1);
            let smallest: usize = segtree.binary_search_right(t, l);
            println!("{}", smallest + 1);
        } else {
            let (r, t) = (queries[query].1 - 1, queries[query].2 - 1);
            let largest: usize = segtree.binary_search_left(t, r);
            println!("{}", largest + 1);
        }
    }
}
