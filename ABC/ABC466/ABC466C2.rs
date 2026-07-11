// use proconio::input_interactive;
// interactive
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufRead, BufReader, Write};
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
    }
    stdout().flush().unwrap();
    // let mut points: Vec<i64> = vec![0; n];
    let mut ans: usize = 0;
    let mut left: usize = 1;
    for i in 0..n - 1 {
        let mut right = left;
        while right < n && query(i, right, &mut source) {
            right += 1;
        }
        let count: usize = right - i;
        ans += count * (count - 1) / 2;
        left = right;
    }
    println!("! {}", ans);
}

fn query<R: BufRead>(i: usize, j: usize, source: &mut LineSource<R>) -> bool {
    println!("? {} {}", i + 1, j + 1);
    stdout().flush().unwrap();
    input! {
        from source,
        ans: String,
    }
    return &ans == "Yes";
}
