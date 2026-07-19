use proconio::input;
use std::io::{stdout, BufWriter, Write};

fn solve() {
    input! {
        t: usize,
    }

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    for _ in 0..t {
        input! {
            px: i128, py: i128,
            qx: i128, qy: i128,
            rx: i128, ry: i128,
            sx: i128, sy: i128,
        }

        let a1 = 2 * (px - qx);
        let b1 = 2 * (py - qy);
        let c1 = (px * px + py * py) - (qx * qx + qy * qy);

        let a2 = 2 * (rx - sx);
        let b2 = 2 * (ry - sy);
        let c2 = (rx * rx + ry * ry) - (sx * sx + sy * sy);

        let parallel = a1 * b2 - a2 * b1 == 0;
        let possible = if parallel {
            a1 * c2 - a2 * c1 == 0 && b1 * c2 - b2 * c1 == 0
        } else {
            true
        };

        if possible {
            writeln!(out, "Yes").unwrap();
        } else {
            writeln!(out, "No").unwrap();
        }
    }
}

fn main() {
    solve();
}
