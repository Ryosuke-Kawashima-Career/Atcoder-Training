use proconio::input;
const MOD: i64 = 998244353;
fn main() {
    input! {n: usize, m: usize, a: [i64; n], b: [i64; m]}
    let mut Bj_modj: i64 = 0;
    for j in 1..=m {
        Bj_modj += b[j - 1] % (j as i64);
        Bj_modj %= MOD;
    }
    let mut ans: i64 = 0;
    for i in 1..=n {
        ans += a[i - 1] * (i as i64);
        ans %= MOD;
    }
    ans *= Bj_modj;
    ans %= MOD;
    println!("{}", ans);
}
