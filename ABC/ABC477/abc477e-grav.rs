use proconio::{input, marker::Usize1};
use std::cmp::min;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        b: [i64; n],
        queries: [(Usize1, Usize1); q],
    }
    let mut dist = vec![0i64; n + 1];
    for i in 0..n {
        dist[i] = b[i];
    }
    dist[n] = 0;
    for _ in 0..2 {
        for i in 0..n {
            let next = (i + 1) % n;
            dist[next] = min(dist[next], dist[i] + a[i]);
        }
    }

    for _ in 0..2 {
        for i in (0..n).rev() {
            let prev = (i + n - 1) % n;
            dist[prev] = min(dist[prev], dist[i] + a[prev]);
        }
    }

    let mut pref = vec![0i64; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + a[i];
    }
    let total_cycle = pref[n];

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for (s, t) in queries {
        if t == n {
            writeln!(out, "{}", dist[s]).unwrap();
        } else if s == n {
            writeln!(out, "{}", dist[t]).unwrap();
        } else {
            let u = min(s, t);
            let v = std::cmp::max(s, t);
            let direct_dist = pref[v] - pref[u];
            let cycle_dist = min(direct_dist, total_cycle - direct_dist);
            let center_dist = dist[s] + dist[t];

            writeln!(out, "{}", min(cycle_dist, center_dist)).unwrap();
        }
    }
}
