use proconio::{input, marker::Chars};
// ABC477C
// Q. Find if the pattern of t exists in l..=r of s.
// A. prefix sum of the pattern occurence.
fn main() {
    input! {q: usize, s: Chars, t: Chars}
    let occurences: Vec<usize> = get_occurence(&s, &t);
    let mut prefix_occurence: Vec<usize> = vec![0; occurences.len() + 1];
    for i in 0..occurences.len() {
        prefix_occurence[i + 1] = prefix_occurence[i] + occurences[i];
    }
    let m: usize = t.len();
    for _query in 0..q {
        input! {l: usize, r: usize}
        if l + m - 1 <= r {
            let occur: usize = prefix_occurence[r + 1 - m] - prefix_occurence[l - 1];
            if occur > 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    }
}

fn get_occurence(text: &[char], pattern: &[char]) -> Vec<usize> {
    let mut occurences: Vec<usize> = vec![0; text.len()];
    let n: usize = text.len();
    let m: usize = pattern.len();
    for i in m - 1..n {
        if pattern == &text[i - m + 1..=i] {
            occurences[i - m + 1] = 1;
        }
    }
    occurences
}
