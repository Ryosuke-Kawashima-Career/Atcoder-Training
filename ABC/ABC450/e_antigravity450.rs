use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        y: Chars,
        q: usize,
        queries: [(u64, u64, char); q],
    }

    // frequencies for whole strings: freq[m][c]
    let mut freq = vec![vec![0u64; 26], vec![0u64; 26], vec![0u64; 26]];
    // lengths of strings: cur_len[m]
    let mut cur_len = vec![0u64, x.len() as u64, y.len() as u64];

    for i in 0..x.len() {
        let c = (x[i] as u8 - b'a') as usize;
        freq[1][c] += 1;
    }
    for i in 0..y.len() {
        let c = (y[i] as u8 - b'a') as usize;
        freq[2][c] += 1;
    }

    // Precalculate Fibonacci strings up to a length safely > 10^18
    let mut m = 2;
    while cur_len[m] < 1_000_000_000_000_000_000 {
        m += 1;
        let l = cur_len[m - 1] + cur_len[m - 2];
        cur_len.push(l);
        
        let mut f = vec![0u64; 26];
        for c in 0..26 {
            f[c] = freq[m - 1][c] + freq[m - 2][c];
        }
        freq.push(f);
    }

    // Prefix sums for base strings X and Y to answer base-case queries in O(1)
    let x_len = x.len();
    let mut pref_x = vec![vec![0u64; 26]; x_len + 1];
    for i in 0..x_len {
        for c in 0..26 {
            pref_x[i + 1][c] = pref_x[i][c];
        }
        pref_x[i + 1][(x[i] as u8 - b'a') as usize] += 1;
    }

    let y_len = y.len();
    let mut pref_y = vec![vec![0u64; 26]; y_len + 1];
    for i in 0..y_len {
        for c in 0..26 {
            pref_y[i + 1][c] = pref_y[i][c];
        }
        pref_y[i + 1][(y[i] as u8 - b'a') as usize] += 1;
    }

    // Answer queries
    for (l, r, c_char) in queries {
        let c_idx = (c_char as u8 - b'a') as usize;
        let ans_r = count(m, r, c_idx, &cur_len, &freq, &pref_x, &pref_y);
        let ans_l = count(m, l - 1, c_idx, &cur_len, &freq, &pref_x, &pref_y);
        println!("{}", ans_r - ans_l);
    }
}

// Recursive function to find the occurrences of a character `c` within the first `k` characters of S_m
fn count(
    m: usize, 
    k: u64, 
    c: usize, 
    cur_len: &[u64], 
    freq: &[Vec<u64>], 
    pref_x: &[Vec<u64>], 
    pref_y: &[Vec<u64>]
) -> u64 {
    if k == 0 {
        return 0;
    }
    if m == 1 {
        return pref_x[k as usize][c];
    }
    if m == 2 {
        return pref_y[k as usize][c];
    }
    if k == cur_len[m] {
        return freq[m][c];
    }

    // S_m is composed of S_{m-1} followed by S_{m-2}
    if k <= cur_len[m - 1] {
        // The prefix perfectly fits inside the left half (S_{m-1})
        return count(m - 1, k, c, cur_len, freq, pref_x, pref_y);
    } else {
        // The prefix covers all of S_{m-1} and spills into S_{m-2}
        let spill_over = k - cur_len[m - 1];
        return freq[m - 1][c] + count(m - 2, spill_over, c, cur_len, freq, pref_x, pref_y);
    }
}
