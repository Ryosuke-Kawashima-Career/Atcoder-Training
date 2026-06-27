use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars};
    let mut east: usize = 0;
    let mut west: usize = 0;
    let n: usize = s.len();
    for i in 0..n {
        if s[i] == 'E' {
            east += 1;
        } else {
            west += 1;
        }
    }
    if east > west {
        println!("East");
    } else {
        println!("West");
    }
}
