use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, s: Chars}
    let mut count: usize = 0;
    for i in 0..n {
        let index: isize = i as isize;
        if s[index as usize] == 'o' {
            continue;
        }
        let index_l: isize = index - 1;
        let index_r: isize = index + 1;
        if index_l < 0 {
            if index_r >= n as isize {
                count += 1;
            } else {
                if s[index_r as usize] == 'x' {
                    count += 1;
                }
            }
        } else if index_r >= n as isize {
            if s[index_l as usize] == 'x' {
                count += 1;
            }
        } else {
            if s[index_l as usize] == 'x' && s[index_r as usize] == 'x' {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
