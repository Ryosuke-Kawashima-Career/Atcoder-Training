use proconio::input;
const MOD: i64 = 998244353;
fn main() {
    input! {n: usize, a: [i64; n]}
    let mut segtree = SegTree::new(&a, segfunc, 0);
}

fn segfunc(node1: i64, node2: i64) -> i64 {}

fn mod_inv(num: i64) -> i64 {
    mod_pow(num, MOD - 2)
}

fn mod_pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    let base_square: i64 = (base * base) % MOD;
    if exp % 2 == 1 {
        return (base * mod_pow(base_square, exp / 2)) % MOD;
    } else {
        return mod_pow(base_square, exp / 2) % MOD;
    }
}

#[derive(Debug)]
pub struct SegTree<T, F>
where
    T: Copy + Eq + std::fmt::Debug,
    F: Fn(T, T) -> T,
{
    data: Vec<T>,
    segfunc: F,
    e: T,
    size: usize,
    height: usize,
}
impl<T, F> SegTree<T, F>
where
    T: Copy + Eq + std::fmt::Debug,
    F: Fn(T, T) -> T,
{
    pub fn new(lis: &Vec<T>, segfunc: F, e: T) -> Self {
        let n0 = lis.len();
        let height = format!("{:b}", n0 - 1).to_string().len();
        let size = 1 << height;
        let mut data = vec![e; size << 1];
        for i in 0..n0 {
            data[size + i] = lis[i];
        }
        for k in (1..=size - 1).rev() {
            let k2 = k << 1;
            data[k] = segfunc(data[k2], data[k2 + 1]);
        }
        Self {
            data,
            segfunc,
            e,
            size,
            height,
        }
    }

    pub fn set(&mut self, idx: usize, value: T) {
        let ni = idx + self.size;
        self.data[ni] = value;
        for k in 1..=self.height {
            let k = ni >> k;
            let k2 = k << 1;
            self.data[k] = (self.segfunc)(self.data[k2], self.data[k2 + 1]);
        }
    }

    pub fn get(&self, idx: usize) -> T {
        self.data[idx + self.size]
    }

    pub fn prod(&self, mut l: usize, mut r: usize) -> T {
        let mut sml = self.e;
        let mut smr = self.e;
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 == 1 {
                sml = (self.segfunc)(sml, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                smr = (self.segfunc)(smr, self.data[r - 1]);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        (self.segfunc)(sml, smr)
    }
    pub fn all_prod(&self) -> T {
        self.data[1]
    }
}
