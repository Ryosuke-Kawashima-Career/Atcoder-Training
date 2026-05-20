use proconio::input;
const MOD: usize = 1_000_000_007;
fn main() {
    input! {m: usize, n: usize}
    let ans: usize = power(m, n);
    println!("{}", ans);
}

fn power(m: usize, n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let m_square: usize = m * m % MOD;
    let result: usize = if n % 2 == 0 {
        power(m_square, n / 2)
    } else {
        (m * power(m_square, (n - 1) / 2)) % MOD
    };
    result % MOD
}
