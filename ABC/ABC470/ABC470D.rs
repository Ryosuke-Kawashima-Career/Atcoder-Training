use proconio::{input, marker::Usize1};
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        n: usize,
        q: usize,
        p_init: [Usize1; n],
    }

    let mut p_direct = p_init;
    let mut p_inverse = vec![0usize; n];
    for i in 0..n {
        p_inverse[p_direct[i]] = i;
    }

    let mut is_inverted = false;

    for _ in 0..q {
        input! {
            qtype: usize,
        }

        if qtype == 1 {
            input! {
                x: Usize1,
                y: Usize1,
            }

            if !is_inverted {
                let val1 = p_direct[x];
                let val2 = p_direct[y];
                p_direct.swap(x, y);
                p_inverse.swap(val1, val2);
            } else {
                let val1 = p_inverse[x];
                let val2 = p_inverse[y];
                p_inverse.swap(x, y);
                p_direct.swap(val1, val2);
            }
        } else {
            is_inverted = !is_inverted;
        }
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let final_p = if !is_inverted { &p_direct } else { &p_inverse };
    for i in 0..n {
        if i > 0 {
            write!(out, " ").unwrap();
        }
        write!(out, "{}", final_p[i] + 1).unwrap();
    }
    writeln!(out).unwrap();
}
