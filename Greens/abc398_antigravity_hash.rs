use proconio::{input, marker::Chars};

// ABC398D
// Problem: Find one shortest palindrome that has S as its prefix.
// Constraints: |S| <= 500,000

// Rolling Hash Solution
// To construct the shortest palindrome starting with S, we need to find the longest valid suffix of S that is a palindrome.
// If the suffix S[i..] is a palindrome, then we just need to append the reverse of S[0..i] to the end.
// We can check if a substring is a palindrome efficiently using Rolling Hash.

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();

    // Use a large prime and a random base to avoid collisions
    // Modulo 2^61 - 1 (Mersenne prime) is standard for 64-bit integers
    const MOD: u64 = (1 << 61) - 1;
    const BASE: u64 = 2023; // Random base usually preferred

    // Helper for modular multiplication (a * b) % (2^61 - 1)
    let mul = |a: u64, b: u64| -> u64 {
        let u128_res = a as u128 * b as u128;
        ((u128_res >> 61) as u64 + (u128_res as u64 & MOD)) % MOD
    };

    // Precompute powers of the base
    let mut power = vec![1; n + 1];
    for i in 0..n {
        power[i + 1] = mul(power[i], BASE);
    }

    // Compute prefix hashes for S (Forward Hash)
    let mut h = vec![0; n + 1];
    for i in 0..n {
        h[i + 1] = (mul(h[i], BASE) + s[i] as u64) % MOD;
    }

    // Compute prefix hashes for S_reversed (Reverse Hash)
    // This allows us to get the hash of the reverse of any substring of S
    let mut h_rev = vec![0; n + 1];
    let s_rev: Vec<char> = s.iter().cloned().rev().collect();
    for i in 0..n {
        h_rev[i + 1] = (mul(h_rev[i], BASE) + s_rev[i] as u64) % MOD;
    }

    // Function to get hash of substring [l, r)
    // h_arr is the prefix hash array
    let get_hash = |h_arr: &Vec<u64>, l: usize, r: usize| -> u64 {
        let val = h_arr[r] + MOD - mul(h_arr[l], power[r - l]);
        val % MOD
    };

    let mut max_len = 0;

    // Check all suffixes of length `len`
    // A suffix of S of length `len` corresponds to S[n-len .. n]
    // Its reverse corresponds to the prefix of S_rev of length `len`, i.e., S_rev[0 .. len]
    // If Hash(S[n-len..n]) == Hash(S_rev[0..len]), it is a palindrome.

    for len in (1..=n).rev() {
        let hash_suffix = get_hash(&h, n - len, n);
        let hash_rev_prefix = get_hash(&h_rev, 0, len);

        if hash_suffix == hash_rev_prefix {
            max_len = len;
            break;
        }
    }

    // Construct the answer
    // Append the reverse of the remaining prefix (S[0 .. n - max_len])
    let mut ans = s.clone();
    for i in (0..n - max_len).rev() {
        ans.push(s[i]);
    }

    println!("{}", ans.iter().collect::<String>());
}
