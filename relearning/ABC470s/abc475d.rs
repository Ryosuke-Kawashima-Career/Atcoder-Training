use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {s: String}
    let s_bytes: Vec<u8> = s.bytes().collect();
    // judge whether the string can be a prime number
    let is_prime = eratosthenes(10usize.pow(s.len() as u32));
    // char -> id
    // Assiging the ID for the each character which appears in s
    let mut dictionary: HashMap<u8, usize> = HashMap::new();
    let mut id: usize = 0;
    for b in s_bytes.iter() {
        dictionary.entry(*b).or_insert(id);
        id += 1;
    }
    let mut pattern: Vec<usize> = Vec::new();
    for b in s_bytes.iter() {
        pattern.push(dictionary[b]);
    }
    // id -> number
    let n_id: usize = dictionary.len();
    let mut id_to_number: Vec<Option<usize>> = vec![None; n_id];
    if dfs(0, &pattern, &is_prime, &mut id_to_number) {
        for id in pattern.iter() {
            print!("{}", id_to_number[*id].unwrap());
        }
        println!();
    } else {
        println!("-1");
    }
}

fn eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime: Vec<bool> = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=limit {
        if is_prime[i] {
            let mut k: usize = 2;
            while i * k <= limit {
                is_prime[i * k] = false;
                k += 1;
            }
        }
    }
    is_prime
}

fn dfs(
    index: usize,
    pattern: &Vec<usize>,
    is_prime: &Vec<bool>,
    id_to_number: &mut Vec<Option<usize>>,
) -> bool {
    if index == pattern.len() {
        let mut num: usize = 0;
        for id in pattern.iter() {
            num = num * 10 + id_to_number[*id].unwrap();
        }
        if is_prime[num] {
            return true;
        } else {
            return false;
        }
    }
    let mut res: bool = false;
    for digit in 0..10 {
        if index == 0 && digit == 0 {
            continue;
        }
        let cur_id: usize = pattern[index];
        if id_to_number[cur_id].is_none() {
            id_to_number[cur_id] = Some(digit);
            if dfs(index + 1, pattern, is_prime, id_to_number) {
                res = true;
            }
            id_to_number[cur_id] = None;
        } else {
            continue;
        }
    }
    return res;
}
