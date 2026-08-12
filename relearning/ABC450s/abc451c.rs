use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, BufWriter, Write};

fn main() {
    input! {
        q: usize,
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut heap = BinaryHeap::new();

    for _ in 0..q {
        input! {
            qtype: usize,
            h: usize,
        }

        if qtype == 1 {
            heap.push(Reverse(h));
        } else {
            while let Some(&Reverse(smallest)) = heap.peek() {
                if smallest <= h {
                    heap.pop();
                } else {
                    break;
                }
            }
        }

        writeln!(out, "{}", heap.len()).unwrap();
    }
}
