use proconio::{input, marker::Chars};
// ABC398D
// Q. Find one shortest palindrome that has S as its prefix.
// Constraint: N <= 5e5
// A. Manacher's algorithm
/* 各 i について「文字 i を中心とする最長の回文の半径」を記録した配列 R を O(|S|) で構築するアルゴリズムです。半径というのは、(全長+1)/2 */
fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    let palindrome_radiuses: Vec<usize> = manacher(&s);
    let mut max_len: usize = 0;

    // The transformed string T has length 2*n + 1. Indices 0..2*n.
    // A palindrome centered at i with radius R extends to i + R.
    // If i + R == 2*n, it touches the right edge -> it's a suffix.
    let len_t = palindrome_radiuses.len();
    for i in 0..len_t {
        if i + palindrome_radiuses[i] == len_t - 1 {
            // max_len in standard string is exactly the radius in transformed string
            if palindrome_radiuses[i] > max_len {
                max_len = palindrome_radiuses[i];
            }
        }
    }

    // We append the reverse of the prefix that is NOT part of the palindromic suffix.
    // Prefix length = N - max_len.
    let mut ans: Vec<char> = s.clone();
    for i in (0..(n - max_len)).rev() {
        ans.push(s[i]);
    }
    println!("{}", ans.iter().collect::<String>());
}

fn manacher(s: &Vec<char>) -> Vec<usize> {
    let mut target: Vec<char> = vec!['#'];
    for c in s {
        target.push(*c);
        target.push('#');
    }
    let n_target: usize = target.len();
    let mut radiuses: Vec<usize> = vec![0; n_target];
    let mut center: usize = 0;
    // the rightmost position of the palindrome centered at center
    let mut right: usize = 0;
    for i in 0..n_target {
        // Use mirror property if i is inside the current right boundary R
        if i < right {
            let mirror: usize = center * 2 - i;
            // i and mirror are the centers of the same palindromes
            radiuses[i] = radiuses[mirror].min(right - i);
        }
        // Attempt to expand around i
        // Check bounds: (i - radisues[i] - 1) and (i + radisues[i] + 1)
        let mut left_edge: isize = i as isize - radiuses[i] as isize - 1;
        let mut right_edge: isize = i as isize + radiuses[i] as isize + 1;

        // BUG FIX: left_edge >= 0 to include index 0
        while left_edge >= 0
            && right_edge < n_target as isize
            && target[left_edge as usize] == target[right_edge as usize]
        {
            radiuses[i] += 1;
            left_edge -= 1;
            right_edge += 1;
        }

        // Update center and right
        // right_edge is now at the mismatch position, so the actual boundary is right_edge - 1
        if i + radiuses[i] > right {
            center = i;
            right = i + radiuses[i];
        }
    }
    radiuses
}
