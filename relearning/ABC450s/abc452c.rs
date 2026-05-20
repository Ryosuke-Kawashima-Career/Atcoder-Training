use proconio::{input, marker::Chars, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, 
        ab: [(usize, Usize1); n], 
        m: usize, 
        s: [Chars; m]
    }

    // Step 1: Pre-process available strings by length.
    let mut length_masks: HashMap<usize, Vec<u32>> = HashMap::new();
    for string in &s {
        let len = string.len();
        let masks = length_masks.entry(len).or_insert_with(|| vec![0; len]);
        for (idx, &ch) in string.iter().enumerate() {
            masks[idx] |= 1 << (ch as u8 - b'a');
        }
    }

    // Step 2: Pre-calculate the constraint for each character of the spine.
    let mut spine_constraints = vec![0u32; n];
    for i in 0..n {
        let (a_len, b_th) = ab[i];
        if let Some(masks) = length_masks.get(&a_len) {
            if b_th < masks.len() {
                spine_constraints[i] = masks[b_th];
            }
        }
    }

    // Step 3: Check each string to see if it can be the spine.
    for query_s in &s {
        let mut is_ok = true;
        if query_s.len() != n {
            is_ok = false;
        } else {
            for (i, &ch) in query_s.iter().enumerate() {
                let char_bit = 1 << (ch as u8 - b'a');
                if char_bit & spine_constraints[i] == 0 {
                    is_ok = false;
                    break;
                }
            }
        }

        if is_ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

