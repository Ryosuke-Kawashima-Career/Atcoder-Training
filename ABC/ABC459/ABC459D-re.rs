use proconio::{input, marker::Chars};
// ABC459 D: No adjacent cells have the same character
// Q. Given a string, find the minimum number of characters to change so that no two
// A. The idea of stack & Greedy algorithm: You only have to choose the most frequent character!!!
fn char_to_index(c: char) -> usize {
    return (c as u8 - b'a') as usize;
}
fn main() {
    input!{t: usize}
    for _case in 0..t {
        input!{s: Chars}
        let n: usize = s.len();
        let mut count: Vec<usize> = vec![0; 26];
        for &c in s.iter() {
            count[char_to_index(c)] += 1;
        }
        
        let max_freq = *count.iter().max().unwrap();
        if max_freq > (n + 1) / 2 {
            println!("No");
            continue;
        }
        
        let mut result: String = String::with_capacity(n);
        let mut last_alpha: usize = 26;
        for _index in 0..n {
            let mut max_count: usize = 0;
            let mut alpha_max: usize = 26;
            for alpha in 0..26 {
                // Prevent adjacent characters from being the same
                if last_alpha != alpha && count[alpha] > max_count {
                    max_count = count[alpha];
                    alpha_max = alpha;
                }
            }
            
            count[alpha_max] -= 1;
            result.push((alpha_max as u8 + b'a') as char);
            last_alpha = alpha_max;
        }
        println!("Yes");
        println!("{}", result);
    }
}