use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        n: usize,
        hw: [(usize, usize); n],
    }

    // Heights max-heap: (height, index)
    let mut heights: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    // Widths max-heap: (width, index)
    let mut widths: BinaryHeap<(usize, usize)> = BinaryHeap::new();

    for (i, &(hi, wi)) in hw.iter().enumerate() {
        heights.push((hi, i));
        widths.push((wi, i));
    }

    let mut used = vec![false; n];
    let mut position = vec![(0, 0); n];

    // Current top-left corner of the remaining bounding box
    let mut top = 0;
    let mut left = 0;

    for _ in 0..n {
        // Discard already placed pieces from the top of the heaps (lazy deletion)
        while let Some(&(_, idx)) = heights.peek() {
            if used[idx] {
                heights.pop();
            } else {
                break;
            }
        }
        while let Some(&(_, idx)) = widths.peek() {
            if used[idx] {
                widths.pop();
            } else {
                break;
            }
        }

        // Case 1: The tallest remaining piece matches the current height of the bounding box
        if let Some(&(max_h, idx)) = heights.peek() {
            if max_h == h {
                heights.pop();
                used[idx] = true;
                position[idx] = (top, left);

                let piece_w = hw[idx].1;
                left += piece_w;
                w -= piece_w;
                continue;
            }
        }

        // Case 2: The widest remaining piece matches the current width of the bounding box
        if let Some(&(max_w, idx)) = widths.peek() {
            if max_w == w {
                widths.pop();
                used[idx] = true;
                position[idx] = (top, left);

                let piece_h = hw[idx].0;
                top += piece_h;
                h -= piece_h;
                continue;
            }
        }

        unreachable!("A valid piece must always match either the current height or width");
    }

    for (r, c) in position {
        println!("{} {}", r + 1, c + 1);
    }
}

