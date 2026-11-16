use proconio::{input, marker::Usize1};
// typical competitive programming problem: Problem 17
// Q. Count the number of intersecting segments
// A. Use Wavelet Matrix to count the number of segments t with l_s < l_t < r_s and r_s < r_t < n for each segment s.
// BitVector for Rank0 and Rank1 Queries
#[derive(Clone)]
struct BitVector {
    // blocks[i] stores bits in the range [i*64, (i+1)*64)
    blocks: Vec<usize>,
    prefix_sum: Vec<usize>,
}

impl BitVector {
    fn new(bits: &[bool]) -> Self {
        let num_blocks = (bits.len() + 63) / 64;
        let mut blocks = vec![0; num_blocks];
        for (i, &b) in bits.iter().enumerate() {
            if b {
                blocks[i / 64] |= 1 << (i % 64);
            }
        }
        let mut prefix_sum = vec![0; num_blocks + 1];
        for i in 0..num_blocks {
            prefix_sum[i + 1] = prefix_sum[i] + blocks[i].count_ones() as usize;
        }
        Self { blocks, prefix_sum }
    }

    #[inline]
    fn rank1(&self, i: usize) -> usize {
        let block_idx = i / 64;
        let bit_idx = i % 64;
        let sum_ones = self.prefix_sum[block_idx];
        let mask = (1 << bit_idx) - 1;
        let count_ones = (self.blocks[block_idx] & mask).count_ones() as usize;
        sum_ones + count_ones
    }

    #[inline]
    fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
    }
}

// Wavelet Matrix for 2D orthogonal range counting
struct WaveletMatrix {
    levels: Vec<BitVector>,
    zeros: Vec<usize>,
    bit_len: usize,
}

impl WaveletMatrix {
    fn new(mut data: Vec<usize>, max_val: usize) -> Self {
        let bit_len = (max_val + 1).next_power_of_two().trailing_zeros() as usize;
        let bit_len: usize = bit_len.max(1);
        let n: usize = data.len();
        // levels[i] stores the BitVector for the i-th bit of the elements in data
        let mut levels: Vec<BitVector> = Vec::with_capacity(bit_len);
        let mut zeros: Vec<usize> = vec![0; bit_len];
        // MSB to LSB by using `rev()` to iterate from bit_len-1 to 0
        for b in (0..bit_len).rev() {
            let mut bits: Vec<bool> = vec![false; n];
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
            // stably partition: 0-bit elements first then 1-bit elements
            Self {
                levels,
                zeros,
                bit_len,
            }
        }
    }

    fn count_less(&self, mut l: usize, mut r: usize, upper: usize) -> usize {
        if l >= r || upper == 0 {
            return 0;
        }
        if upper >= (1usize << self.bit_len) {
            return r - l;
        }
        let mut count: usize = 0;
        for (idx, bv) in self.levels.iter().enumerate() {
            let b = (upper >> (self.bit_len - 1 - idx)) & 1;
            if b == 1 {
                count += bv.rank0(r) - bv.rank0(l);
                l = self.zeros[idx] + bv.rank1(l);
                r = self.zeros[idx] + bv.rank1(r);
            } else {
                l = bv.rank0(l);
                r = bv.rank0(r);
            }
        }
        count
    }

    fn range_freq(&self, l: usize, r: usize, val_l: usize, val_r: usize) -> usize {
        // Count elements in data[l..r) whose values lie in [val_l, val_r)
        self.count_less(l, r, val_r) - self.count_less(l, r, val_l)
    }
}

fn main() {
    input! {n: usize, m: usize, mut lr: [(Usize1, Usize1); m]}
    // Sort by the end
    lr.sort_by_key(|key| (key.1, key.0));
    // Build a Wavelet matrix over the right endpoints R
    let r_vals: Vec<usize> = lr.iter().map(|&(_, r)| r).collect();
    let wm = WaveletMatrix::new(r_vals, n);
    // Count segment t with: l_s < l_t < r_s and r_s < r_t < n;
    let mut ans: usize = 0;
    for &(l_s, r_s) in lr.iter() {
        // Find index range [idx_l, idx_r) where l_s + 1 <= l_t < r_s
        let idx_l = lr.partition_point(|&(l, _)| l < l_s + 1);
        let idx_r = lr.partition_point(|&(l, _)| l < r_s);
        ans += wm.range_freq(idx_l, idx_r, r_s + 1, n);
    }
    println!("{}", ans);
}
