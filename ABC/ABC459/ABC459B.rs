use proconio::{input, marker::Chars};

fn get_number(c: char) -> usize {
    match c {
        'a'..='c' => 2,
        'd'..='f' => 3,
        'g'..='i' => 4,
        'j'..='l' => 5,
        'm'..='o' => 6,
        'p'..='s' => 7,
        't'..='v' => 8,
        'w'..='z' => 9,
        _ => 0,
    }
}

fn main() {
    input! {n: usize, s: [Chars; n]}
    for i in 0..n {
        print!("{}", get_number(s[i][0]));
    }
    println!("");
}
