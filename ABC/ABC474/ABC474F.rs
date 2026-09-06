use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    // 1. Linear Sieve to compute Möbius function mu up to n
    let mut mu = vec![0i64; n + 1];
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();

    mu[1] = 1;
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            if i * p > n {
                break;
            }
            is_prime[i * p] = false;
            if i % p == 0 {
                mu[i * p] = 0;
                break;
            } else {
                mu[i * p] = -mu[i];
            }
        }
    }

    // 2. Compute prefix sums of mu: M[m] = sum_{k=1}^m mu[k]
    let mut m_pref = vec![0i64; n + 1];
    for i in 1..=n {
        m_pref[i] = m_pref[i - 1] + mu[i];
    }

    // 3. For each j in 1..=n, compute S_j = sum_{k=1}^{floor(n/j)} mu[k] * A[k * j]
    // The condition c_j >= 0 translates to:
    // X * M(floor(n/j)) >= S_j
    let mut lower_bound: i128 = *a.iter().max().unwrap() as i128;
    let mut upper_bound: i128 = i128::MAX;

    for j in 1..=n {
        let m = n / j;
        let mut s: i128 = 0;
        for k in 1..=m {
            s += (mu[k] as i128) * (a[k * j - 1] as i128);
        }

        let mj = m_pref[m] as i128;

        if mj > 0 {
            // X >= ceil(s / mj) = -((-s).div_euclid(mj))
            let bound = -((-s).div_euclid(mj));
            lower_bound = lower_bound.max(bound);
        } else if mj < 0 {
            // X * mj >= s  <=>  X * (-mj) <= -s
            // X <= floor(-s / (-mj)) = (-s).div_euclid(-mj)
            let bound = (-s).div_euclid(-mj);
            upper_bound = upper_bound.min(bound);
        } else {
            // mj == 0: requires 0 >= s
            if s > 0 {
                println!("-1");
                return;
            }
        }
    }

    if lower_bound <= upper_bound {
        let min_x = lower_bound;
        // Total operations = Delta A_1 = X - A_1
        let ans = min_x - (a[0] as i128);
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

