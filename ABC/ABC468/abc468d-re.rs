use proconio::{input, marker::Chars};
// ABC468D
// Q. Find the number of pseudo-palindromes(1 mismatch is acceptable)
// A. Divide cases by Even and Odd lengths
fn main() {
    input! {s: Chars}
    let mut ans: usize = 0;
    // if the length of palindrome is odd
    ans += get_odd_palindromes(&s, 1);
    ans += get_even_palindromes(&s, 1);
    println!("{}", ans);
}

fn get_even_palindromes(s: &Vec<char>, threshold: usize) -> usize {
    /* Get the number of palindromes with the center index: i */
    let n: usize = s.len();
    let mut result: usize = 0;
    for pivot in 0..n - 1 {
        let mut radius: usize = 0;
        let mut diff_cnt: usize = 0;
        loop {
            let l: isize = pivot as isize - radius as isize;
            let r: isize = pivot as isize + 1 + radius as isize;
            if l < 0 || r >= n as isize {
                break;
            }
            if s[l as usize] == s[r as usize] {
                radius += 1;
            } else {
                diff_cnt += 1;
                if diff_cnt > threshold {
                    break;
                }
                radius += 1;
            }
        }
        result += radius;
    }
    result
}

fn get_odd_palindromes(s: &Vec<char>, threshold: usize) -> usize {
    /* Get the number of palindromes with the center index: i */
    let n: usize = s.len();
    let mut result: usize = 0;
    for pivot in 0..n {
        let mut radius: usize = 1;
        let mut diff_cnt: usize = 0;
        loop {
            let l: isize = pivot as isize - radius as isize;
            let r: isize = pivot as isize + radius as isize;
            if l < 0 || r >= n as isize {
                break;
            }
            if s[l as usize] == s[r as usize] {
                radius += 1;
            } else {
                diff_cnt += 1;
                if diff_cnt > threshold {
                    break;
                }
                radius += 1;
            }
        }
        result += radius;
    }
    result
}
