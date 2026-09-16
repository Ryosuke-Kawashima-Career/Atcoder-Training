use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let n = s.len();
    let s_bytes = s.as_bytes();

    // Map each distinct character in s to an ID in 0..k
    let mut char_to_id = std::collections::HashMap::new();
    let mut pattern = Vec::with_capacity(n);
    let mut k = 0;

    for &b in s_bytes {
        let id = *char_to_id.entry(b).or_insert_with(|| {
            let cur = k;
            k += 1;
            cur
        });
        pattern.push(id);
    }

    // Precompute primes up to 10^n using Sieve of Eratosthenes
    let mut limit = 1usize;
    for _ in 0..n {
        limit *= 10;
    }

    let mut is_prime = vec![true; limit];
    if limit > 0 {
        is_prime[0] = false;
    }
    if limit > 1 {
        is_prime[1] = false;
    }
    let mut p = 2;
    while p * p < limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple < limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }

    // Backtracking / DFS to assign distinct digits 0..=9 to each ID 0..k
    let mut assigned = vec![None; k];
    let mut used_digit = [false; 10];

    fn dfs(
        id: usize,
        k: usize,
        pattern: &[usize],
        assigned: &mut [Option<u8>],
        used_digit: &mut [bool; 10],
        is_prime: &[bool],
    ) -> Option<usize> {
        if id == k {
            // Reconstruct the decimal number
            let mut val = 0usize;
            for &char_id in pattern {
                let d = assigned[char_id].unwrap() as usize;
                val = val * 10 + d;
            }
            if is_prime[val] {
                return Some(val);
            }
            return None;
        }

        for digit in 0..=9 {
            // First character cannot be mapped to 0 (no leading zero)
            if id == 0 && digit == 0 {
                continue;
            }
            if !used_digit[digit as usize] {
                used_digit[digit as usize] = true;
                assigned[id] = Some(digit);

                if let Some(res) = dfs(id + 1, k, pattern, assigned, used_digit, is_prime) {
                    return Some(res);
                }

                assigned[id] = None;
                used_digit[digit as usize] = false;
            }
        }

        None
    }

    if let Some(ans) = dfs(
        0,
        k,
        &mut pattern,
        &mut assigned,
        &mut used_digit,
        &is_prime,
    ) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
