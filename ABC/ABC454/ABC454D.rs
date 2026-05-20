use proconio::{input, marker::Chars};
use std::collections::VecDeque;
// Q. Replace (xx) => xx and xx => (xx) | Is string A adjustable to string B?
fn main() {
    input! {t: usize}
    let mut answer: Vec<String> = Vec::new();
    for _case in 0..t {
        input! {a: Chars, b: Chars}
        if adjustable(&a, &b) {
            answer.push("Yes".to_string());
        } else {
            answer.push("No".to_string());
        }
    }
    for ans in answer {
        println!("{}", ans);
    }
}

fn trim(s: &Vec<char>) -> VecDeque<char> {
    let mut result: VecDeque<char> = VecDeque::new();
    let n: usize = s.len();
    let mut is_there_xx: bool = false;
    for i in 0..n {
        result.push_back(s[i]);
        if is_there_xx {
            if result.len() >= 2 {
                if result[result.len() - 2] == '(' && result[result.len() - 1] == ')' {
                    result.pop_back();
                    result.pop_back();
                } else {
                    is_there_xx = false;
                }
            }
        }
        if result.len() >= 4
            && result[result.len() - 4] == '('
            && result[result.len() - 3] == 'x'
            && result[result.len() - 2] == 'x'
            && result[result.len() - 1] == ')'
        {
            result.pop_back();
            result.pop_back();
            result.pop_back();
            result.pop_back();
            is_there_xx = true;
        }
    }
    result
}

fn adjustable(a: &Vec<char>, b: &Vec<char>) -> bool {
    let a_trimmed: VecDeque<char> = trim(a);
    let b_trimmed: VecDeque<char> = trim(b);
    a_trimmed == b_trimmed
}
