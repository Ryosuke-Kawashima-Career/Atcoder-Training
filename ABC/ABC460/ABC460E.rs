use proconio::input;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
// 主客転倒型の問題では?
fn solve() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u64,
            m: u64,
        }

        let mut ans: u128 = 0;
        let mut ten_pow: u128 = 1;

        for _l in 1..=19 {
            let prev_ten = ten_pow;
            if let Some(next_ten) = ten_pow.checked_mul(10) {
                ten_pow = next_ten;
            } else {
                break;
            }

            if prev_ten > n as u128 {
                break;
            }

            let min_val = prev_ten;
            let max_val = std::cmp::min(n as u128, ten_pow - 1);
            if min_val > max_val {
                continue;
            }
            let c_l = max_val - min_val + 1;

            let d = ten_pow - 1;
            // Since M <= 10^9 and d can be up to 10^18 - 1, we can compute d % M
            // and find the gcd of (d % M) and M.
            let rem_d = (d % (m as u128)) as u64;
            let g = gcd(rem_d, m);
            let m_div_g = m / g;
            let k_l = n / m_div_g;

            let contribution = ((c_l % 998244353) * (k_l as u128 % 998244353)) % 998244353;
            ans = (ans + contribution) % 998244353;
        }

        println!("{}", ans);
    }
}

fn main() {
    solve();
}

