use proconio::input;
use std::io::{self, Write};

fn main() {
    // (shift, modulus)
    input!{n: usize, ab: [(usize, usize); n], q: usize, queries: [usize; q]}
    let max_x: usize = *queries.iter().max().unwrap();

    // (x + a) mod b = (x + a) - b * floor((x + a) / b)
    // f(x) := sum b * floor((x + a) / b). The floor increases by 1 exactly at x = s + k * b,
    // where s = b - a is in 1..=b. Then f(0) = 0 because a < b.
    let mut sum_a: u64 = 0;
    let mut bs: Vec<(usize, usize)> = Vec::with_capacity(n);
    for &(a, b) in ab.iter() {
        sum_a += a as u64;
        bs.push((b, b - a));
    }
    bs.sort_unstable();

    // diff[x] = f(x) - f(x - 1)
    let mut diff: Vec<u64> = vec![0; max_x + 1];
    let mut i: usize = 0;
    while i < n {
        let b: usize = bs[i].0;
        // Same (b, s) pairs jump at the same x, so they are merged into (s, weight).
        let mut jumps: Vec<(usize, u64)> = Vec::new();
        while i < n && bs[i].0 == b {
            let s: usize = bs[i].1;
            let mut count: u64 = 0;
            while i < n && bs[i] == (b, s) {
                count += 1;
                i += 1;
            }
            jumps.push((s, count * b as u64));
        }

        if jumps.len() * 8 >= b {
            // Dense: add a table of length b block by block (sequential access, vectorized).
            let mut table: Vec<u64> = vec![0; b];
            for &(s, weight) in jumps.iter() {
                table[s % b] += weight;
            }
            for block in diff.chunks_mut(b) {
                for (d, &t) in block.iter_mut().zip(table.iter()) {
                    *d += t;
                }
            }
        } else {
            // Sparse: visit each arithmetic progression directly.
            for &(s, weight) in jumps.iter() {
                let mut x: usize = s;
                while x <= max_x {
                    diff[x] += weight;
                    x += b;
                }
            }
        }
    }
    // The dense table also writes to x = 0, which is not a jump (s >= 1).
    diff[0] = 0;

    // Prefix sum: f[x] = f(x)
    let mut f: Vec<u64> = diff;
    for x in 1..=max_x {
        f[x] += f[x - 1];
    }

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for &x in queries.iter() {
        let ans: u64 = x as u64 * n as u64 + sum_a - f[x];
        writeln!(out, "{}", ans).unwrap();
    }
}
