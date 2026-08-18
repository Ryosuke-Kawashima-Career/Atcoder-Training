use proconio::{input, marker::Chars};

fn main() {
    input!{n: usize, s: Chars}
    let mut shakutori: Vec<usize> = vec![0; n];
    let mut left: usize = 0;
    let mut count_hit: usize = 0;
    for k in 0..n {
        let mut right: usize = left.max(k);
        while right < n && count_hit > 0 {
            if s[right] == 'o' {
                count_hit += 1;
            } else {
                count_hit -= 1;
            }
            right += 1;
        }
        shakutori[k] = right + 1;
        if s[left] == 'o' {
            count_hit -= 1;
        } else {
            count_hit += 1;
        }
        left += 1;
    }

    for i in 0..n {
        println!("{}", shakutori[i]);
    }
}
