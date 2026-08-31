use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, l: usize, r: usize, s: Chars}
    let mut alphas: Vec<Vec<usize>> = vec![vec![]; 26];
    for i in 0..n {
        alphas[s[i] as usize - 'a' as usize].push(i);
    }
    let mut ans: usize = 0;
    for alpha in 0..26 {
        for &index in alphas[alpha].iter() {
            let index_l: usize = alphas[alpha].partition_point(|&x| x < index + l);
            let index_r: usize = alphas[alpha].partition_point(|&x| x <= index + r);
            ans += index_r.saturating_sub(index_l);
        }
    }
    println!("{ans}");
}
