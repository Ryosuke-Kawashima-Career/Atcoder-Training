use proconio::{input, marker::Bytes};
// Typical 047
// Q. Find the number of diagonals with the same colors(if s[i]==t[j], then the color is s[i] | otherwise choose the color different from the two.)
// A. Express colors with numerals (Encoding)
fn to_num(b: u8) -> u8 {
    match b {
        b'R' => 0,
        b'G' => 1,
        b'B' => 2,
        _ => unreachable!(),
    }
}

fn z_algorithm(a: &[u8]) -> Vec<usize> {
    let n = a.len();
    let mut z = vec![0; n];
    z[0] = n;
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i < r {
            z[i] = z[i - l].min(r - i);
        }
        while i + z[i] < n && a[z[i]] == a[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

fn count_suffix_prefix_matches(prefix_src: &[u8], suffix_src: &[u8], start_offset: usize) -> usize {
    let n = prefix_src.len();
    let mut combined = Vec::with_capacity(2 * n + 1);
    combined.extend_from_slice(prefix_src);
    combined.push(3); // sentinel not in {0, 1, 2}
    combined.extend_from_slice(suffix_src);

    let z = z_algorithm(&combined);
    let mut count = 0;
    for d in start_offset..n {
        if z[n + 1 + d] == n - d {
            count += 1;
        }
    }
    count
}

fn main() {
    input! {
        _n: usize,
        s_bytes: Bytes,
        t_bytes: Bytes,
    }

    let s: Vec<u8> = s_bytes.into_iter().map(to_num).collect();
    let t: Vec<u8> = t_bytes.into_iter().map(to_num).collect();

    let mut ans = 0;
    for c in 0..3u8 {
        // Cell (i, j) has color c <=> -(s[i] + t[j]) == c (mod 3) <=> t[j] == (3 + 3 - c - s[i]) % 3
        let sc: Vec<u8> = s.iter().map(|&si| (6 - c - si) % 3).collect();

        // k >= 0: T[k..n] == sc[0..n-k]
        ans += count_suffix_prefix_matches(&sc, &t, 0);

        // k < 0 (d = -k >= 1): sc[d..n] == T[0..n-d]
        ans += count_suffix_prefix_matches(&t, &sc, 1);
    }

    println!("{}", ans);
}
