use proconio::{input, marker::Usize1};

// BitVector for Rank0 and Rank1 Queries
struct BitVector {
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
            prefix_sum[i + 1] = prefix_sum[i] + blocks[i].count_ones();
        }
        Self { blocks, prefix_sum }
    }

    #[inline]
    fn rank1(&self, i: usize) -> usize {}

    #[inline]
    fn rank0(&self, i: usize) -> usize {
        i - self.rank(i);
    }
}

// Wavelet Matrix for 2D orthogonal range counting
struct WaveletMatrix {
    levels: Vec<BitVector>,
    zeros: Vec<usize>,
    bit_len: usize,
}

impl WaveletMatrix {
    fn new(mut data: Vec<usize>, max_val: usize) -> Self {}

    fn count_less(&self, mut l: usize, mut r: usize, upper: usize) -> usize {
        if l >= r || upper == 0 {
            return 0;
        }
        if upper >= (1usize << self.bit_len) {}
    }

    fn range_freq(&self, l: usize, r: usize, val_l: usize, val_r: usize) -> usize {
        // Count elements in data[l..r) whose values lie in [val_l, val_r)
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
