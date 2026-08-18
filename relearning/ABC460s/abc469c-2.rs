use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // Positions of all 'x' characters (0-indexed)
    let mut x_indices: Vec<usize> = Vec::new();
    for (i, &c) in s.iter().enumerate() {
        if c == 'x' {
            x_indices.push(i);
        }
    }

    // For each k in 0..n:
    // When flipping at most k 'x's into 'o's starting from index i,
    // how long can the contiguous sequence of 'o's be?
    // If there are at most k 'x's in the entire string, the answer is n.
    // Otherwise, we find the (k+1)-th 'x' at or after index i.
    for k in 0..n {
        let total_x = x_indices.len();
        if total_x <= k {
            println!("{}", n);
        } else {
            // Find the index of the (k+1)-th 'x' from the beginning
            // (i.e. x_indices[k])
            let ans = x_indices[k];
            println!("{}", ans+1);
        }
    }
}
