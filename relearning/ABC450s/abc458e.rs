use proconio::input;
const MOD: i64 = 998244353;
fn main() {
    input!{x1: i64, x2: i64, x3: i64}
    let x_sum: i64 = x1 + x2 + x3;
    let mut factorial: Vec<i64> = vec![1; x_sum as usize +1];
    let mut factorial_inv: Vec<i64> = vec![1; x_sum as usize +1];
    for num in 1..=x_sum {
        factorial[num as usize] = num * factorial[num as usize -1];
        factorial[num as usize] %= MOD;
        factorial_inv[num as usize] = power(factorial[num as usize], MOD-2);
    }
    let nCr = |n: i64, r: i64| -> i64 {
        factorial[n as usize] * factorial_inv[r as usize] % MOD * factorial[(n-r) as usize]  
    };

    let choose_2: i64 = nCr(x_sum-x1.max(x3), x2) % MOD;
    let choose_3: i64 = nCr(x2+1, x3) % MOD;
    let ans: i64 = choose_2 * choose_3 % MOD;
    println!("{}", ans);
}

fn power(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    let base_square: i64 = (base * base) % MOD;
    let result: i64 = if exp % 2 == 0 {
        power(base_square, exp / 2)
    } else {
        base * power(base_square, (exp - 1) / 2) % MOD
    };
    return result % MOD;
}
