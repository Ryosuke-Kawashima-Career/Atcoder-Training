use proconio::input;
use std::collections::HashMap;
// abc475d
// Q. Find a prime number whose pattern matches that of string `s`.
// A. brute force attack
fn main() {
    input! {s: String}
    let n: usize = s.len();
    let s_bytes = s.as_bytes();
    let mut char_to_id = HashMap::new();
    // string -> id
    let mut pattern: Vec<usize> = Vec::with_capacity(n);
    // total number of ids
    let mut k: usize = 0;

    // switch from char to id
    for &b in s_bytes {
        let id: usize = *char_to_id.entry(b).or_insert_with(|| {
            let cur = k;
            k += 1;
            cur
        });
        pattern.push(id);
    }
    let limit: usize = 10usize.pow(n as u32);
    let is_prime: Vec<bool> = judge_prime(limit);
    // dfs for the brute force attack
    // assgined[id]: digit which is assigned to id
    let mut assigned: Vec<Option<usize>> = vec![None; k];
    let mut used_digit: [bool; 10] = [false; 10];
    if let Some(ans) = dfs(0, k, &pattern, &mut assigned, &mut used_digit, &is_prime) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn dfs(
    id: usize,
    k: usize,
    pattern: &Vec<usize>,
    assigned: &mut Vec<Option<usize>>,
    used_digit: &mut [bool; 10],
    is_prime: &Vec<bool>,
) -> Option<usize> {
    // edge case
    if id == k {
        // reconstruct the decimal number
        let mut val: usize = 0;
        for &p_id in pattern {
            let assigned_id = assigned[p_id].unwrap();
            val = 10 * val + assigned_id;
        }
        if is_prime[val] {
            return Some(val);
        }
        return None;
    }

    for digit in 0..10 {
        // prevent the leading zero
        if id == 0 && digit == 0 {
            continue;
        }
        // prevent the reuse of the digit
        if used_digit[digit] {
            continue;
        }
        assigned[id] = Some(digit);
        used_digit[digit] = true;
        if let Some(ans) = dfs(id + 1, k, pattern, assigned, used_digit, is_prime) {
            return Some(ans);
        }
        // back tracking
        assigned[id] = None;
        used_digit[digit] = false;
    }

    None
}

fn judge_prime(n: usize) -> Vec<bool> {
    // Sieve of Eratosthenes
    let mut is_prime: Vec<bool> = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for p in 2..=n {
        if is_prime[p] {
            let mut k: usize = 2;
            while k * p <= n {
                is_prime[k * p] = false;
                k += 1;
            }
        }
    }
    is_prime
}
