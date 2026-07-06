use proconio::input;
const MOD: usize = 998244353;
fn main() {
    input! {t: usize}
    let mut power10: Vec<usize> = vec![1; 20];
    for exp in 1..20 {
        power10[exp] = power10[exp - 1] * 10 % MOD;
    }
    for _case in 0..t {
        input! {n: usize, m: usize}
        let ans: usize = solve(n, m, &power10);
        println!("{}", ans);
    }
}

fn solve(n: usize, m: usize, power10: &Vec<usize>) -> usize {
    let n_str: String = n.to_string();
    let digits: usize = n_str.len();
    let mut result: usize = 0;
    for digit in 1..digits {
        let power10_minus_1_mod: usize = (power10[digit] % m + m - 1) % m;
        let count: usize = n / (power10_minus_1_mod / gcd(n, power10_minus_1_mod));
        result += (count) % MOD;
        result %= MOD;
    }
    result % MOD
}

fn gcd(n: usize, m: usize) -> usize {
    if n == 0 || m == 0 {
        return m + n;
    }
    gcd(m, n % m)
}
