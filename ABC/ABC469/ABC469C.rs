use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, s: Chars}
    let mut x_indexes: Vec<usize> = Vec::new();
    for i in 0..n {
        if s[i] == 'x' {
            x_indexes.push(i);
        }
    }
    for k in 0..n {
        if x_indexes.len() < k {
            println!("{}", n);
        } else {
            let target: usize = x_indexes.partition_point(|&x| x < k);
            if target < x_indexes.len() {
                let ans: usize = x_indexes[target] + 1;
                println!("{}", ans);
            } else {
                println!("0");
            }
        }
    }
}
