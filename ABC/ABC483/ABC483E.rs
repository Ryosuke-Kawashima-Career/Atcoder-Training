use proconio::{input, marker::Bytes};
use std::collections::HashMap;

fn main() {
    input! {n: usize, m: usize, s: [Bytes; n]}

    // Six lowercase letters fit in 30 bits, with five bits per position.
    let encoded: Vec<u32> = s
        .iter()
        .map(|word| {
            word.iter().enumerate().fold(0_u32, |value, (i, &c)| {
                value | (u32::from(c - b'a') << (5 * i))
            })
        })
        .collect();

    let mut answer = 0_i64;
    let mut counts = HashMap::<u32, i64>::with_capacity(n);
    for subset in 0_usize..(1 << m) {
        let kept = subset.count_ones() as usize;
        if kept < m - 2 {
            continue;
        }
        // Correct the overcounting of pairs agreeing in more than m - 2 places.
        let coefficient = if kept == m - 2 {
            1
        } else if kept == m - 1 {
            -((m - 2) as i64)
        } else {
            ((m - 1) * (m - 2) / 2) as i64
        };
        if coefficient == 0 {
            continue;
        }

        let mut bit_mask = 0_u32;
        for i in 0..m {
            if subset & (1 << i) != 0 {
                bit_mask |= 31_u32 << (5 * i);
            }
        }

        counts.clear();
        let mut pairs = 0_i64;
        for &word in &encoded {
            let count = counts.entry(word & bit_mask).or_insert(0);
            pairs += *count;
            *count += 1;
        }
        answer += coefficient * pairs;
    }
    println!("{}", answer);
}
