// interactive
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufRead, BufReader, Write};
// shakutori
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {from &mut source, n: usize}
    let mut ans: usize = 0;
    let mut left: usize = 0;
    for i in 0..n {
        let mut right: usize = (i + 1).max(left);
        while right < n && query(i, right, &mut source) {
            right += 1;
        }
        ans += (right - i).saturating_sub(1);
        left = right;
    }
    println!("! {}", ans);
}

fn query<R: BufRead>(i: usize, j: usize, source: &mut LineSource<R>) -> bool {
    println!("? {} {}", i + 1, j + 1);
    stdout().flush().unwrap();
    input! {from source, x: String}
    x == "Yes"
}
