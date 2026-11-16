use proconio::{input, marker::Usize1};

/// Succinct BitVector supporting O(1) rank0 and rank1 queries.
struct BitVector {
    blocks: Vec<u64>,
    prefix_sum: Vec<u32>,
}

impl BitVector {
    fn new(bits: &[bool]) -> Self {
        let num_blocks = (bits.len() + 63) / 64;
        let mut blocks = vec![0u64; num_blocks];
        for (i, &b) in bits.iter().enumerate() {
            if b {
                blocks[i / 64] |= 1u64 << (i % 64);
            }
        }
        let mut prefix_sum = vec![0u32; num_blocks + 1];
        for i in 0..num_blocks {
            prefix_sum[i + 1] = prefix_sum[i] + blocks[i].count_ones();
        }
        Self { blocks, prefix_sum }
    }

    /// Number of 1s in bits[0..i)
    #[inline]
    fn rank1(&self, i: usize) -> usize {
        let block_idx = i / 64;
        let bit_idx = i % 64;
        let mut res = self.prefix_sum[block_idx] as usize;
        if bit_idx > 0 {
            let mask = (1u64 << bit_idx) - 1;
            res += (self.blocks[block_idx] & mask).count_ones() as usize;
        }
        res
    }

    /// Number of 0s in bits[0..i)
    #[inline]
    fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
    }
}

/// Wavelet Matrix supporting O(log(max_val)) 2D orthogonal range counting.
struct WaveletMatrix {
    // WaveletMatrix is for ranking, counting, and selection of elements in a range.
    levels: Vec<BitVector>,
    zeros: Vec<usize>,
    bit_len: usize,
}

impl WaveletMatrix {
    fn new(mut data: Vec<usize>, max_val: usize) -> Self {
        let bit_len = (max_val + 1).next_power_of_two().trailing_zeros() as usize;
        let bit_len = bit_len.max(1);
        let n = data.len();
        let mut levels = Vec::with_capacity(bit_len);
        let mut zeros = Vec::with_capacity(bit_len);

        for b in (0..bit_len).rev() {
            let mut bits = vec![false; n];
            let mut zero_vals = Vec::new();
            let mut one_vals = Vec::new();

            for (i, &x) in data.iter().enumerate() {
                if (x >> b) & 1 == 1 {
                    bits[i] = true;
                    one_vals.push(x);
                } else {
                    zero_vals.push(x);
                }
            }

            zeros.push(zero_vals.len());
            levels.push(BitVector::new(&bits));

            // Stably partition: 0-bit elements first, then 1-bit elements
            zero_vals.extend(one_vals);
            data = zero_vals;
        }

        Self {
            levels,
            zeros,
            bit_len,
        }
    }

    /// Count elements in data[l..r) that are strictly less than `upper`.
    fn count_less(&self, mut l: usize, mut r: usize, upper: usize) -> usize {
        if l >= r || upper == 0 {
            return 0;
        }
        if upper >= (1usize << self.bit_len) {
            return r - l;
        }

        let mut count = 0;
        for (idx, bv) in self.levels.iter().enumerate() {
            let b = self.bit_len - 1 - idx;
            let l0 = bv.rank0(l);
            let r0 = bv.rank0(r);
            let l1 = bv.rank1(l);
            let r1 = bv.rank1(r);

            if (upper >> b) & 1 == 1 {
                // All elements with 0 at bit `b` are strictly < `upper`
                count += r0 - l0;
                // Continue into the 1-bit partition
                l = self.zeros[idx] + l1;
                r = self.zeros[idx] + r1;
            } else {
                // Continue into the 0-bit partition
                l = l0;
                r = r0;
            }
        }
        count
    }

    /// Count elements in data[l..r) whose values lie in [val_l, val_r).
    fn range_freq(&self, l: usize, r: usize, val_l: usize, val_r: usize) -> usize {
        if l >= r || val_l >= val_r {
            return 0;
        }
        self.count_less(l, r, val_r) - self.count_less(l, r, val_l)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(Usize1, Usize1); m],
    }

    // Step 1: Sort segments by left endpoint L ascending
    lr.sort_unstable_by_key(|&(l, r)| (l, r));

    // Step 2: Build Wavelet Matrix over the right endpoints R
    let r_vals: Vec<usize> = lr.iter().map(|&(_, r)| r).collect();
    let wm = WaveletMatrix::new(r_vals, n);

    // Step 3: For each segment (l_s, r_s), count segments t with:
    // l_s < l_t < r_s  AND  r_s < r_t < n
    let mut ans: usize = 0;
    for &(l_s, r_s) in &lr {
        // Find index range [idx_l, idx_r) where l_s + 1 <= l_t < r_s
        let idx_l = lr.partition_point(|&(l, _)| l < l_s + 1);
        let idx_r = lr.partition_point(|&(l, _)| l < r_s);

        // Count how many in [idx_l, idx_r) have r_t in [r_s + 1, n)
        ans += wm.range_freq(idx_l, idx_r, r_s + 1, n);
    }

    println!("{}", ans);
}
