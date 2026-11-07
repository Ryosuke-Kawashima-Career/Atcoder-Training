use proconio::input;
use std::fmt::Write;

fn main() {
    input! {n: usize, m: usize, ab: [(usize, usize); m]}
    let mut candidate_first_votes: Vec<usize> = vec![0; n + 1];
    let mut to_seconds: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for i in 0..m {
        candidate_first_votes[ab[i].0] += 1;
        to_seconds[ab[i].0].push(ab[i].1);
    }
    // Scanning in increasing order keeps the smaller number first on ties.
    let (mut first, mut second) = (1, 2);
    if candidate_first_votes[second] > candidate_first_votes[first] {
        std::mem::swap(&mut first, &mut second);
    }
    for candidate in 3..=n {
        if candidate_first_votes[candidate] > candidate_first_votes[first] {
            second = first;
            first = candidate;
        } else if candidate_first_votes[candidate] > candidate_first_votes[second] {
            second = candidate;
        }
    }

    let mut output = String::new();
    for k in 1..=n {
        let mut winner = if k == first { second } else { first };

        // Apply every transfer before comparing final vote totals.
        for &candidate in &to_seconds[k] {
            candidate_first_votes[candidate] += 1;
        }
        for &candidate in &to_seconds[k] {
            if candidate_first_votes[candidate] > candidate_first_votes[winner]
                || (candidate_first_votes[candidate] == candidate_first_votes[winner]
                    && candidate < winner)
            {
                winner = candidate;
            }
        }
        writeln!(output, "{}", winner).unwrap();

        // Restore the original counts for the next withdrawal scenario.
        for &candidate in &to_seconds[k] {
            candidate_first_votes[candidate] -= 1;
        }
    }
    print!("{}", output);
}
