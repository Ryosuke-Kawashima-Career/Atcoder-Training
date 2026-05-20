use proconio::{input, marker::Chars};
const MOD: i64 = 1_000_000_007;
// ABC398D
// Q. Find one shortest palindrome that has S as its prefix.
// Constraint: N <= 5e5
// A. Palindrome is usually solved with reversing the string!!!
fn main() {
    input! {s: Chars}
    let s_rev = s.clone().into_iter().rev().collect::<Vec<char>>();
    let hash_s: Vec<i64> = get_hash(&s);
    let hash_s_rev: Vec<i64> = get_hash(&s_rev);
    let n: usize = s.len();
    for i in 1..=n {
        if hash_s[i] == hash_s_rev[n - i] {
            let mut ans: Vec<char> = Vec::new();
            for j in 0..i {
                ans.push(s[j]);
            }
            for j in (0..i - 1).rev() {
                ans.push(s_rev[j]);
            }
            println!("{}", ans.iter().collect::<String>());
            return;
        }
    }
}

fn get_hash(s: &Vec<char>) -> Vec<i64> {
    /*
    hash[i] := s[0] * 26^(i-1) + s[1] * 26^(i-2) + ... + s[i-1] * 26^0
    */
    let n: usize = s.len();
    let mut hash: Vec<i64> = vec![0; n + 1];
    let mut power: Vec<i64> = vec![0; n + 1];
    power[0] = 1;
    for i in 1..=n {
        hash[i] = hash[i - 1] * 26 + (s[i - 1] as i64 - 'a' as i64 + 1);
        if hash[i] < 0 {
            hash[i] += MOD;
        }
        hash[i] %= MOD;
        power[i] = power[i - 1] * 26;
        if power[i] < 0 {
            power[i] += MOD;
        }
        power[i] %= MOD;
    }
    return hash;
}

#[allow(dead_code)]
fn judge_palindrome(s: &Vec<char>) -> bool {
    let n: usize = s.len();
    if n == 1 {
        return true;
    }
    let mut l: usize = 0;
    while n >= l + 1 && 2 * l + 1 <= n {
        if s[l] != s[n - 1 - l] {
            return false;
        }
        l += 1;
    }
    return true;
}
