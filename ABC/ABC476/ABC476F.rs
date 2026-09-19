use proconio::input;

fn main() {
    input! {n: usize, mod_m: usize, a: [usize; n], b: [usize; n]}
    let mut residents: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            residents[i][j] = (a[i] * b[j]) % mod_m;
        }
    }
}

fn cost(sr: usize, tr: usize, sc: usize, tc: usize) -> usize {
    let row_value: usize = (sr as isize - tr as isize).abs() as usize;
    let col_value: usize = (sc as isize - tc as isize).abs() as usize;
    row_value.max(col_value)
}

fn bit_wise_xor(i: usize, j: usize) -> usize {}
