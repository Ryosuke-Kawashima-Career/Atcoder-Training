use proconio::input;
// ABC470C
// Q. 1 x: Increase the value of Ax by 1.
// 2: For each i=1,2,…,N, if Ai≥1, decrease the value of Ai by 1.
// A. Store the indexes which changed its status.
// XOR: self-inverse
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
        /* Toggle the value at index idx in the Fenwick tree
        0 <-> 1
        */
        idx += 1;
        let len = self.tree.len();
        while idx < len {
            self.tree[idx] ^= 1;
            /* Add the lsb of idx to idx */
            idx += idx & (!idx + 1);
        }
    }

    #[inline(always)]
    fn query_prefix(&self, mut idx: usize) -> u8 {
        idx += 1;
        let mut res = 0;
        while idx > 0 {
            res ^= self.tree[idx];
            // Calculate lsb
            idx &= idx - 1;
        }
        res
    }

    #[inline(always)]
    fn query_range(&self, l: usize, r: usize) -> u8 {
        /* Returns the parity of [l r] */
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
    input! {n: usize, q: usize}

    let max_val = q + 5;
    let mut parity: Vec<u8> = vec![0u8; max_val + 1];
    let mut b: Vec<usize> = vec![0; n + 1];
    // bits[k: k-th bit, from 0]
    let mut bits: Vec<FenwickXor> = (0..19).map(|k| FenwickXor::new(1 << (k + 1))).collect();
    let mut shift: usize = 0;

    for _ in 0..q {
        input! {qtype: usize}
        if qtype == 1 {
            // Add one to a[x]
            input! {x: usize}
            let prev_value: usize = b[x];
            let new_value: usize = prev_value.max(shift) + 1;
            b[x] = new_value;
            if prev_value > shift {
                parity[prev_value] ^= 1;

                for keta in 0..19 {
                    let size: usize = 1 << (keta + 1);
                    // erase the old value
                    bits[keta].toggle(prev_value % size);
                }
            }
            parity[new_value] ^= 1;
            for keta in 0..19 {
                let size: usize = 1 << (keta + 1);
                // register the new value
                bits[keta].toggle(new_value % size);
            }
        } else {
            // Turn the values > 0 to values - 1, then calculate the nim
            shift += 1;
            if parity[shift] == 1 {
                parity[shift] = 0;
                for keta in 0..19 {
                    let size: usize = 1 << (keta + 1);
                    bits[keta].toggle(shift % size);
                }
            }
        }
        let mut nim: usize = 0;
        for keta in 0..19 {
            let size: usize = 1 << (keta + 1);
            let half_size: usize = 1 << keta;
            let start: usize = (shift + half_size) % size;
            let end: usize = (shift + size - 1) % size;
            let partiy: u8 = if start <= end {
                bits[keta].query_range(start, end)
            } else {
                bits[keta].query_range(start, size - 1) ^ bits[keta].query_prefix(end)
            };
            if partiy == 1 {
                nim |= 1 << keta;
            }
        }
        println!("{}", nim);
    }
}
