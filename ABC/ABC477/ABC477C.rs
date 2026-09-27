use proconio::{input, marker::Chars};

fn main() {
    // s: Target t: Pattern
    input!{q: usize, s: Chars, t: Chars}
    let lps: Vec<usize> = get_lps_array(&t);
    let matches: Vec<usize> = kmp_count(&s, &t, &lps)
    for _query in 0..q {
        input!{l: usize, r: usize}
        let mut is_ok: bool = false;
        for &match_idx in matches.iter() {
            let match_idx_right: usize = match_idx + t.len() - 1;
            if l <= match_idx && match_idx_right <= r {
                is_ok = true;
                break;
            }
        }
        if is_ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn get_lps_array(pattern: &Vec<char>) -> Vec<usize> {
    // lps: Least prefix suffix
    let m: usize = pattern.len();
    let mut lps: Vec<usize> = vec![0; m];
    let mut length: usize = 0;
    let mut i: usize = 1;
    while i < m {
        if pattern[i] == pattern[length] {
            length += 1;
            lps[i] = length;
            i += 1;
        } else {
            if length != 0 {
                length = lps[length - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    return lps;
}

fn kmp_count(text: &Vec<char>, pattern: &Vec<char>, lps: &Vec<usize>) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();
    // index for text
    let mut i: usize = 0;
    // index for pattern
    let mut j: usize = 0;
    let mut matches: Vec<usize> = Vec::new();

    while i < n {
        if text[i] == pattern[j] {
            i += 1;
            j += 1;
        }
        if j == m {
            matches.push(i - j);
            j = lps[j-1];
        } else if i < n && text[i] != pattern[j] {
            if j != 0 {
                j = lps[j-1];
            } else {
                i += 1;
            }
        }
    }
    return matches;
}
