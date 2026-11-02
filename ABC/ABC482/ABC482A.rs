use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}
    let n: usize = s.len();
    let mut count_o: usize = 0;
    for i in 0..n {
        if s[i] == 'o' {
            count_o += 1;
        }
    }
    if count_o % 2 == 1 {
        println!("odd");
    } else {
        println!("even");
    }
}
