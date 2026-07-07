use proconio::{input, marker::Usize1};

struct Fenwick {
    data: Vec<isize>,
}
impl Fenwick {
    fn new(n: usize) -> Self {
        // 1-indexed BIT over positions 1..=n
        Self {
            data: vec![0; n + 1],
        }
    }
    fn add(&mut self, mut i: usize, v: isize) {
        // requires i >= 1
        while i < self.data.len() {
            self.data[i] += v;
            i += i & i.wrapping_neg();
        }
    }
    fn sum(&self, mut i: usize) -> isize {
        // prefix sum over positions 1..=i
        let mut res = 0;
        while i > 0 {
            res += self.data[i];
            i -= i & i.wrapping_neg();
        }
        res
    }
}

fn main() {
    input! { n: usize, q: usize }
    let mut count: Vec<usize> = vec![0; n];
    let mut foundation: usize = 0;

    // raw count of any cell is at most q, so positions 1..=q+1 (raw_count + 1) suffice
    let mut ft = Fenwick::new(q + 1);
    ft.add(1, n as isize); // everyone starts at raw count 0 -> position 1

    for _ in 0..q {
        input! { query_type: usize }
        if query_type == 1 {
            input! { x: Usize1 } // 0-indexed cell
            let pre = count[x];
            ft.add(pre + 1, -1);
            count[x] += 1;
            let post = count[x];
            ft.add(post + 1, 1);
            if ft.sum(foundation + 1) == 0 {
                foundation += 1;
            }
        } else {
            input! { y: usize }
            let k = y + foundation;
            let idx = k.min(q + 1); // clamp: k > q means the answer is 0
            println!("{}", ft.sum(q + 1) - ft.sum(idx));
        }
    }
}
