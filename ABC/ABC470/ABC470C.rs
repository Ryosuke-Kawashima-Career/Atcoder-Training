use proconio::{input, marker::Usize1};
use std::io::{self, BufWriter, Write};

struct FenwickXor {
    tree: Vec<u8>,
}

impl FenwickXor {
    fn new(power_of_two: usize) -> Self {
        FenwickXor {
            tree: vec![0; power_of_two + 1],
        }
    }

    #[inline(always)]
    fn toggle(&mut self, mut idx: usize) {
        idx += 1;
        let len = self.tree.len();
        while idx < len {
            self.tree[idx] ^= 1;
            idx += idx & (!idx + 1);
        }
    }

    #[inline(always)]
    fn query_prefix(&self, mut idx: usize) -> u8 {
        idx += 1;
        let mut res = 0;
        while idx > 0 {
            res ^= self.tree[idx];
            idx &= idx - 1;
        }
        res
    }

    #[inline(always)]
    fn query_range(&self, l: usize, r: usize) -> u8 {
        if l > r {
            return 0;
        }
        let r_val = self.query_prefix(r);
        if l == 0 {
            r_val
        } else {
            r_val ^ self.query_prefix(l - 1)
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let max_val = q + 5;
    let mut p = vec![0u8; max_val + 1];
    let mut b = vec![0usize; n + 1];
    let mut bits: Vec<FenwickXor> = (0..19).map(|k| FenwickXor::new(1 << (k + 1))).collect();

    let mut s = 0usize;

    for _ in 0..q {
        input! {
            qtype: usize,
        }

        if qtype == 1 {
            input! {
                x: Usize1,
            }
            let x = x + 1; // 1-indexed internally

            let old_v = b[x];
            let new_v = old_v.max(s) + 1;
            b[x] = new_v;

            if old_v > s {
                p[old_v] ^= 1;
                for k in 0..19 {
                    let sz = 1 << (k + 1);
                    bits[k].toggle(old_v % sz);
                }
            }

            p[new_v] ^= 1;
            for k in 0..19 {
                let sz = 1 << (k + 1);
                bits[k].toggle(new_v % sz);
            }
        } else {
            s += 1;
            if p[s] == 1 {
                p[s] = 0;
                for k in 0..19 {
                    let sz = 1 << (k + 1);
                    bits[k].toggle(s % sz);
                }
            }
        }

        let mut xor_sum = 0usize;
        for k in 0..19 {
            let sz = 1 << (k + 1);
            let half = 1 << k;
            let start = (s + half) % sz;
            let end = (s + sz - 1) % sz;

            let parity = if start <= end {
                bits[k].query_range(start, end)
            } else {
                bits[k].query_range(start, sz - 1) ^ bits[k].query_range(0, end)
            };

            if parity == 1 {
                xor_sum |= 1 << k;
            }
        }

        writeln!(out, "{}", xor_sum).unwrap();
    }
}
