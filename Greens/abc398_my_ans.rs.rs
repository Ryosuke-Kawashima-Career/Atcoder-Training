use proconio::{input, marker::Chars};
// ABC398D
// Q. Find one shortest palindrome that has S as its prefix.
// Constraint: N <= 5e5
// A. Palindrome is usually solved with reversing the string!!!
fn main() {
    input! {s: Chars}
    let s_rev = s.clone().into_iter().rev().collect::<Vec<char>>();
    let hash_s: Hash = Hash::new(&s);
    let hash_s_rev: Hash = Hash::new(&s_rev);
    let n: usize = s.len();
    let mut max_len: usize = 0;

    // We want the longest suffix of S that is a palindrome.
    // A suffix of S corresponds to S[n-len .. n].
    // Its reverse is S_rev[0 .. len].
    for len in (1..=n).rev() {
        if hash_s.get_hash(n - len, n) == hash_s_rev.get_hash(0, len) {
            max_len = len;
            break;
        }
    }

    let mut ans: Vec<char> = s.clone();
    // Append the reverse of the remaining prefix (which is S[0 .. n-max_len])
    // The problem asks for shortest palindrome starting with S.
    // T = S + (Reverse of S excluding the palindromic suffix)
    for i in (0..(n - max_len)).rev() {
        ans.push(s[i]);
    }
    println!("{}", ans.iter().collect::<String>());
}

struct Hash {
    string: Vec<char>,
    hash: Vec<i64>,
    power: Vec<i64>,
}

impl Hash {
    // Use a large prime and a random base to avoid collisions
    // Modulo 2^61 - 1 (Mersenne prime) is standard for 64-bit integers
    const MOD: i64 = 1_000_000_007;
    const BASE: i64 = 26; // Random base usually preferred
    fn new(target_s: &Vec<char>) -> Self {
        let n: usize = target_s.len();
        let s: Vec<char> = target_s.clone();
        let mut hash: Vec<i64> = vec![0; n + 1];
        let mut power: Vec<i64> = vec![0; n + 1];
        power[0] = 1;
        for i in 1..=n {
            // Map 'a' -> 1 to avoid 'a' vs 'aa' collision (0 vs 0*B+0)
            hash[i] = hash[i - 1] * Self::BASE + (s[i - 1] as i64 - 'a' as i64 + 1);
            hash[i] %= Self::MOD;
            power[i] = power[i - 1] * Self::BASE;
            power[i] %= Self::MOD;
        }
        Self {
            string: s,
            hash,
            power,
        }
    }

    fn get_hash(&self, l: usize, r: usize) -> i64 {
        // Correct modular subtraction:
        // (hash[r] - hash[l] * power[r-l]) % MOD
        // We calculate the term separately and ensure positive result
        let term = (self.hash[l] * self.power[r - l]) % Self::MOD;
        let val = (self.hash[r] + Self::MOD - term) % Self::MOD;
        val
    }
}
