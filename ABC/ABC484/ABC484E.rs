use proconio::input;

struct Fenwick {
    n: usize,
    tree: Vec<u64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            n,
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut i: usize, delta: u64) {
        i += 1;
        while i <= self.n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    /// Returns the sum of elements in `[0, r)`.
    fn prefix_sum(&self, mut r: usize) -> u64 {
        let mut s = 0;
        while r > 0 {
            s += self.tree[r];
            r -= r & r.wrapping_neg();
        }
        s
    }

    /// Returns the sum of elements in `[l, r)`.
    fn range_sum(&self, l: usize, r: usize) -> u64 {
        if l >= r {
            0
        } else {
            self.prefix_sum(r) - self.prefix_sum(l)
        }
    }
}

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }

    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }

    let mut vals = prefix.clone();
    vals.sort_unstable();
    vals.dedup();

    let mut bit = Fenwick::new(vals.len());
    let mut count: u64 = 0;

    for &p in &prefix {
        let target_l = p - r;
        let target_r = p - l;

        let left = vals.partition_point(|&x| x < target_l);
        let right = vals.partition_point(|&x| x <= target_r);
        count += bit.range_sum(left, right);

        let idx = vals.binary_search(&p).unwrap();
        bit.add(idx, 1);
    }

    println!("{}", count);
}
