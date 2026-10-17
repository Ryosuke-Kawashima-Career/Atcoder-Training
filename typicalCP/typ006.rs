use proconio::{input, marker::Chars};
fn main() {
    input! {n: usize, k: usize, s: Chars}
    let mut next_alpha_indexes: Vec<Vec<usize>> = vec![vec![n; n + 1]; 26];
    for index in (0..n).rev() {
        for alpha in 0..26 {
            if s[index] as usize - 'a' as usize == alpha {
                next_alpha_indexes[alpha][index] = index;
            } else {
                next_alpha_indexes[alpha][index] = next_alpha_indexes[alpha][index + 1];
            }
        }
    }
    let mut current_index: usize = 0;
    let mut result: Vec<char> = Vec::new();
    for _ in 0..k {
        for alpha in 0..26 {
            if next_alpha_indexes[alpha][current_index] < n {
                let next_index: usize = next_alpha_indexes[alpha][current_index];
                let possible_length: usize = n - next_index;
                if possible_length >= k - result.len() {
                    result.push(('a' as u8 + alpha as u8) as char);
                    current_index = next_index + 1;
                    break;
                }
            }
        }
    }
    println!("{}", result.iter().collect::<String>());
}
