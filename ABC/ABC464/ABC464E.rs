use proconio::input;
use std::io::{stdout, BufWriter, Write};

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        rcx: [(usize, usize, char); q],
    }

    let mut char_map = vec!['A'; q + 1];
    let mut max_op = vec![0usize; h * w];

    for i in 0..q {
        let (r, c, x) = rcx[i];
        char_map[i + 1] = x;
        // 0-based indices
        let r_idx = r - 1;
        let c_idx = c - 1;
        let idx = r_idx * w + c_idx;
        max_op[idx] = std::cmp::max(max_op[idx], i + 1);
    }

    // Propagate the operation indices to the top and left
    for r in (0..h).rev() {
        for c in (0..w).rev() {
            let idx = r * w + c;
            let mut val = max_op[idx];
            if r + 1 < h {
                val = std::cmp::max(val, max_op[(r + 1) * w + c]);
            }
            if c + 1 < w {
                val = std::cmp::max(val, max_op[r * w + (c + 1)]);
            }
            max_op[idx] = val;
        }
    }

    // Output the resulting grid using fast I/O
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for r in 0..h {
        let mut row_chars = String::with_capacity(w);
        for c in 0..w {
            let op_idx = max_op[r * w + c];
            row_chars.push(char_map[op_idx]);
        }
        writeln!(out, "{}", row_chars).unwrap();
    }
}
