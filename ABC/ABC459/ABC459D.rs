use proconio::{input, marker::Chars};
use std::collections::BTreeSet;

fn alpha_to_num(c: char) -> usize {
    return (c as u8 - 'a' as u8) as usize;
}

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {s: Chars}
        let mut count: Vec<usize> = vec![0; 26];
        for &c in s.iter() {
            count[alpha_to_num(c)] += 1;
        }
        let (is_ok, result) = solve(s.len(), &count);
        if is_ok {
            println!("Yes");
            println!("{}", result);
        } else {
            println!("No");
        }
    }
}

fn solve(n: usize, count: &Vec<usize>) -> (bool, String) {
    let mut result: Vec<char> = vec![' '; n];
    let mut available_indexes = BTreeSet::new();
    for i in 0..n {
        available_indexes.insert(i);
    }
    for alpha in 0..26usize {
        let c: char = (alpha as u8 + 'a' as u8) as char;
        let min_index: usize = *available_indexes.iter().min().unwrap();
        for diff in 0..count[alpha] {
            let index: usize = min_index + 2 * diff;
            if index >= n {
                return (false, "".to_string());
            }
            result[index] = c;
            available_indexes.remove(&index);
        }
    }
    return (true, result.iter().collect::<String>());
}
