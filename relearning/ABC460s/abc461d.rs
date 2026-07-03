use proconio::{input, marker::Chars};
// abc461
fn main() {
    input! {h: usize, w: usize, k: usize, s: [Chars; h]}
    let mut prefix: Vec<Vec<usize>> = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '1' {
                prefix[i + 1][j + 1] = 1;
            }
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            prefix[i][j] += prefix[i][j - 1];
        }
    }

    let mut ans: usize = 0;
    for span_col in 1..=w {
        for start_col in 0..=w {
            let end_col: usize = start_col + span_col;
            if end_col > w {
                continue;
            }
            let mut count_one: usize = 0;
            let mut left: usize = 0;
            while left < h {
                let mut right: usize  
            }
        }
    }
}
