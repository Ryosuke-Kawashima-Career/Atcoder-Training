use proconio::input;
const MOD: usize = 1_000_000_007;
fn main() {
    input! {n: usize, b: usize, k: usize, c: [usize; k]}
    // digits[digit][mod]
    let mut digits: Vec<Vec<usize>> = vec![vec![0; b]; n + 1];
    digits[0][0] = 1;
    for digit in 1..=n {
        for mod_val in 0..b {
            for digit_c in &c {
                digits[digit][(mod_val * b + digit_c) % b] += digits[digit - 1][mod_val];
                digits[digit][(mod_val * b + digit_c) % b] %= MOD;
            }
        }
    }
    println!("{}", digits[n][0] % MOD);
}
