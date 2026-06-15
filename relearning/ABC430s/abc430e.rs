// string matching algorithm
use proconio::{input, marker::Chars};
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {a: Chars, b: Chars}
        let double_a: Vec<char> = a.iter().cloned().chain(a.iter().cloned()).collect();
        let a_len = a.len();
        let b_len = b.len();
        if a_len < b_len {
            println!("-1");
            continue;
        }
        let lps = get_lps(&b);
        let matched: Vec<usize> = kmp_search(&double_a, &b, &lps);
        if matched.len() == 0 {
            println!("-1");
            continue;
        }
        let ans: usize = matched[0];
        println!("{}", ans);
    }
}

fn get_lps(pattern: &Vec<char>) -> Vec<usize> {
    /* Return the longest proper prefix (the prefixes match the suffixes)
    Args:
        pattern(&Vec<char>): the patter to match the target with.
    Returns:
        Vec<usize>: the length of the longest proper prefix that matches the suffixes.
    */
    let m: usize = pattern.len();
    let mut lps: Vec<usize> = vec![0; m];
    let mut index: usize = 1;
    // length of previous longest proper prefix suffix
    let mut length: usize = 0;
    while index < m {
        if pattern[index] == pattern[length] {
            length += 1;
            lps[index] = length;
            index += 1;
        } else {
            if length != 0 {
                length = lps[length - 1];
            } else {
                lps[index] = 0;
                index += 1;
            }
        }
    }
    return lps;
}

fn kmp_search(text: &Vec<char>, pattern: &Vec<char>, lps: &Vec<usize>) -> Vec<usize> {
    /* Returns the matched index by Knuth Moris Algorithm
    Args:
        text(&Vec<char>): the target text.
        pattern(&Vec<char>): the pattern to match the target with.
        lps(&Vec<usize>): the length of the longest proper prefix that matches the suffixes.
    Returns:
        Vec<usize>: the matched index.
     */
    let mut matched: Vec<usize> = Vec::new();
    let mut text_index: usize = 0;
    let mut pattern_index: usize = 0;
    let n: usize = text.len();
    let m: usize = pattern.len();

    while text_index < n {
        while pattern_index < m && text_index < n {
            if text[text_index] == pattern[pattern_index] {
                text_index += 1;
                pattern_index += 1;
            } else {
                if pattern_index != 0 {
                    // Return back the previous common length
                    pattern_index = lps[pattern_index - 1];
                } else {
                    // start from scratch
                    text_index += 1;
                }
            }
        }
        if pattern_index == m {
            let first_index: usize = text_index - pattern_index;
            matched.push(first_index);
        }
    }
    return matched;
}
