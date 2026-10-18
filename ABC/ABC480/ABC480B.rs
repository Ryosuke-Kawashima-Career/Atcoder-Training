use proconio::{input, marker::Chars};

/// Computes the Hamming distance between two character slices of equal length.
fn hamming_distance(a: &[char], b: &[char]) -> usize {
    a.iter().zip(b.iter()).filter(|&(ca, cb)| ca != cb).count()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    let mut votes = vec![0usize; n];

    for ballot in &t {
        let mut exact_match = None;
        let mut diff_one_matches = Vec::new();

        for (i, candidate) in s.iter().enumerate() {
            // Two strings must have the same length to match in 0 or 1 character replacement
            if candidate.len() != ballot.len() {
                continue;
            }

            let dist = hamming_distance(candidate, ballot);
            if dist == 0 {
                exact_match = Some(i);
                break; // Candidate names are distinct, so at most one exact match exists
            } else if dist == 1 {
                diff_one_matches.push(i);
            }
        }

        if let Some(i) = exact_match {
            votes[i] += 1;
        } else if diff_one_matches.len() == 1 {
            votes[diff_one_matches[0]] += 1;
        }
    }

    for v in votes {
        println!("{}", v);
    }
}
