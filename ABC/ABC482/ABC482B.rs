use proconio::{input, marker::Chars};

fn main() {
    input! {s: [Chars; 5]}
    let mut count_row: Vec<usize> = vec![0; 5];
    let mut count_col: Vec<usize> = vec![0; 5];
    let mut count_cross_left: usize = 0;
    let mut count_cross_right: usize = 0;
    for i in 0..5 {
        for j in 0..5 {
            if s[i][j] == 'o' {
                count_row[i] += 1;
                count_col[j] += 1;
                if i == j {
                    count_cross_left += 1;
                } else if i + j == 4 {
                    count_cross_right += 1;
                }
            }
        }
    }
    let mut ans: usize = (5 - count_cross_right).min(5 - count_cross_left);
    for i in 0..5 {
        ans = ans.min(5 - count_row[i]);
        ans = ans.min(5 - count_col[i]);
    }
    println!("{}", ans);
}
