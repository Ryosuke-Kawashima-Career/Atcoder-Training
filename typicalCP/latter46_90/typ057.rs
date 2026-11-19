use proconio::{input, marker::Usize1};
const MOD: usize = 998244353;
// Typical CP problem 57
// Q. Find the number of operations to switch [0; m] into s by xor
// A. Gaussian Elimination and Row Reduction
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; m]; n];
    for row in 0..n {
        input! {
            t: usize,
            a: [Usize1; t],
        }
        for col in a {
            matrix[row][col] = 1;
        }
    }
    input! {
        mut s: [u8; m],
    }

    let rank = gaussian_elimination(&mut matrix, &mut s);
    if s.iter().all(|&x| x == 0) {
        let mut ans: usize = 1;
        for _ in rank..n {
            ans = (ans * 2) % MOD;
        }
        println!("{}", ans);
    } else {
        println!("0");
    }
}

fn gaussian_elimination(matrix: &mut [Vec<u8>], vector: &mut [u8]) -> usize {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut pos = 0;

    for col in 0..m {
        let mut found = false;
        for row in pos..n {
            if matrix[row][col] == 1 {
                if row != pos {
                    matrix.swap(row, pos);
                }
                found = true;
                break;
            }
        }
        if found {
            for row in (pos + 1)..n {
                if matrix[row][col] == 1 {
                    for k in col..m {
                        matrix[row][k] ^= matrix[pos][k];
                    }
                }
            }
            if vector[col] == 1 {
                for k in col..m {
                    vector[k] ^= matrix[pos][k];
                }
            }
            pos += 1;
        }
    }
    pos
}
