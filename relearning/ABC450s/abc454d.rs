use proconio::{input, marker::Chars};
use std::io::{self, BufWriter, Write};

fn reduce(s: &[char]) -> Vec<char> {
    let mut stack = Vec::with_capacity(s.len());
    for &ch in s {
        stack.push(ch);
        while stack.len() >= 4 {
            let len = stack.len();
            if stack[len - 4] == '(' && stack[len - 3] == 'x' && stack[len - 2] == 'x' && stack[len - 1] == ')' {
                stack.truncate(len - 4);
                stack.push('x');
                stack.push('x');
            } else {
                break;
            }
        }
    }
    stack
}

fn main() {
    input! {
        t: usize,
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for _ in 0..t {
        input! {
            a: Chars,
            b: Chars,
        }

        let a_canonical = reduce(&a);
        let b_canonical = reduce(&b);

        if a_canonical == b_canonical {
            writeln!(out, "Yes").unwrap();
        } else {
            writeln!(out, "No").unwrap();
        }
    }
}
