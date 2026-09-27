use proconio::{input, marker::Bytes};
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        q: usize,
        s: Bytes,
        t: Bytes,
        queries: [(usize, usize); q],
    }

    let n = s.len();
    let m = t.len();

    let mut pref = vec![0; n + 1];

    if m <= n {
        for i in 0..=(n - m) {
            if s[i..i + m] == t[..] {
                pref[i + 1] = 1;
            }
        }
    }

    for i in 0..n {
        pref[i + 1] += pref[i];
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for (l, r) in queries {
        if r + 1 >= l + m {
            let start = l - 1;
            let end = r - m;
            let count = pref[end + 1] - pref[start];
            if count > 0 {
                writeln!(out, "Yes").unwrap();
            } else {
                writeln!(out, "No").unwrap();
            }
        } else {
            writeln!(out, "No").unwrap();
        }
    }
}
