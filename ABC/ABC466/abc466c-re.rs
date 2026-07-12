use proconio::input;
// interactive
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufRead, BufReader, Write};
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input!{from &mut source, n: usize}
    stdout().flush().unwrap();
    let mut ans: usize = 0;
    let mut left: usize = 1;
    for i in 0..n {
        let mut right: usize = (i + 1).max(left);
        while right < n && query(&mut source, i, right) {
            right += 1;
        }
        ans += right - 1 - i;
        left = right;
    }
    println!("! {}", ans);
}

fn query<R: BufRead>(source: &mut LineSource<R>, l: usize, r: usize) -> bool {
    println!("? {} {}", l+1, r+1);
    stdout().flush().unwrap();
    input!{from source, ans: String}
    return &ans == "Yes";
}
