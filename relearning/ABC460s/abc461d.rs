use proconio::{input, marker::Chars};
// abc461
fn main() {
    input! {h: usize, w: usize, k: usize, s: [Chars; h]}
    let mut vertical: Vec<Vec<usize>> = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '1' {
                vertical[i + 1][j + 1] = 1;
            }
        }
    }
    // vertical prefix sum of each column
    for i in 1..=h {
        for j in 1..=w {
            vertical[i][j] += vertical[i - 1][j];
        }
    }

    let mut ans: usize = 0;
    for up in 1..=h {
        for down in up..=h {
            let mut horizon: Vec<usize> = vec![0; w + 1];
            for col in 1..=w {
                horizon[col] = horizon[col - 1] + (vertical[down][col] - vertical[up - 1][col]);
            }
            ans += sweeping(k, &horizon);
        }
    }
    println!("{}", ans);
}

fn sweeping(k: usize, horizon: &Vec<usize>) -> usize {
    /*Calculates Shakutori
    Args:
        k(usize): the number of one
        horizon(&Vec<usize>): the prefix of each column
     */
    let w: usize = horizon.len();
    let mut r1: usize = 0;
    let mut r2: usize = 0;
    let mut result: usize = 0;
    for left in 0..w - 1 {
        r1 = r1.max(left + 1);
        r2 = r2.max(left + 1);
        while r1 < w && horizon[r1] - horizon[left] < k {
            r1 += 1;
        }
        while r2 < w && horizon[r2] - horizon[left] <= k {
            r2 += 1;
        }
        result += r2 - r1;
    }
    return result;
}
