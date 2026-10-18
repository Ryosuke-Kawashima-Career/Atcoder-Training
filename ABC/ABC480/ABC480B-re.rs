use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, m: usize, s: [Chars; n], t: [Chars; m]}
    let mut votes: Vec<usize> = vec![0; n];
    for ballot in t.iter() {
        let mut exact_match_idx: Option<usize> = None;
        let mut diff_one_matches: Vec<usize> = Vec::new();
        for (i, candidate) in s.iter().enumerate() {
            if ballot.len() != candidate.len() {
                continue;
            }
            let distance: usize = hamming_distance(ballot, candidate);
            if distance == 0 {
                exact_match_idx = Some(i);
                break;
            } else if distance == 1 {
                diff_one_matches.push(i);
            }
        }
        if let Some(i) = exact_match_idx {
            votes[i] += 1;
        } else if diff_one_matches.len() == 1 {
            votes[diff_one_matches[0]] += 1;
        }
    }
    for v in votes {
        println!("{}", v);
    }
}

fn hamming_distance(s: &[char], t: &[char]) -> usize {
    let mut count: usize = 0;
    for (c1, c2) in s.iter().zip(t.iter()) {
        if c1 != c2 {
            count += 1;
        }
    }
    count
}
