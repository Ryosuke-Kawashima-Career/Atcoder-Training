use proconio::{input, marker::Chars};
// ABC398D
// Problem: Find one shortest palindrome that has S as its prefix.
/*  A. The previous hashing approach was incomplete, and the most efficient way to find the longest palindromic suffix is to use the KMP (Knuth–Morris–Pratt) algorithm.

First, construct a temporary string T by concatenating the reversed string S, a separator character ?, and the original string S.

Next, compute the LPS (Longest Prefix Suffix) array, often denoted as pi, for the string T.

The value of the LPS array at the final index of T represents the length of the longest prefix of T that is also a suffix of T.

The prefix of T corresponds to a prefix of the reversed string S, while the suffix of T corresponds to a suffix of the original string S.

If these match, it means that a suffix of S is equal to its own reverse, which implies that the suffix is a palindrome.

Using this method, the length of the longest palindromic suffix can be computed in O(N) time.
*/
fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();

    // Strategy: Find the longest palindromic suffix of S using KMP.
    // Construct T = S_rev + "?" + S.
    // The match at the end gives the length of the longest palindrome at the end of S.

    let mut t = s.clone();
    t.reverse();
    t.push('?'); // Safe separator for uppercase input
    t.extend(&s);

    let m = t.len();
    let mut pi = vec![0; m];
    let mut j = 0;

    // Compute KMP pi array
    for i in 1..m {
        while j > 0 && t[i] != t[j] {
            j = pi[j - 1];
        }
        if t[i] == t[j] {
            j += 1;
        }
        pi[i] = j;
    }

    let k = pi[m - 1]; // Length of the longest palindromic suffix

    // Append the reverse of the prefix that isn't part of the palindrome
    let mut ans = s.clone();
    for i in (0..n - k).rev() {
        ans.push(s[i]);
    }

    println!("{}", ans.iter().collect::<String>());
}
