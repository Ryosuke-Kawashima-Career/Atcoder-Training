use proconio::input;
// Typical Competitive Programming Problems 005
const MOD: usize = 1_000_000_007;
const MAX_LOG: usize = 62;
// Q. Find the number of n-digit numbers that are divisible by b, using digits c[0..k-1]
// A. Digit DP and Doubling
fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k],
    }

    // power_of_10[i] = 10^(2^i) % b
    let mut power_of_10: Vec<usize> = vec![0; MAX_LOG];
    power_of_10[0] = 10 % b;
    for i in 1..MAX_LOG {
        power_of_10[i] = (power_of_10[i - 1] * power_of_10[i - 1]) % b;
    }

    // doubling[i][r] = number of (2^i)-digit numbers congruent to r (mod b)
    let mut doubling: Vec<Vec<usize>> = vec![vec![0; b]; MAX_LOG];
    for &digit in &c {
        doubling[0][digit % b] += 1;
    }

    // Build doubling table
    run_doubling(&mut doubling, &power_of_10);

    // Combine powers of 2 to form length n
    let result: Vec<usize> = get_result(n, &doubling, &power_of_10);
    println!("{}", result[0]);
}

fn run_doubling(doubling: &mut Vec<Vec<usize>>, power_of_10: &[usize]) {
    let max_log = doubling.len();
    let mod_b = doubling[0].len();
    for i in 1..max_log {
        for p in 0..mod_b {
            if doubling[i - 1][p] == 0 {
                continue;
            }
            for q in 0..mod_b {
                let next_mod = (p * power_of_10[i - 1] + q) % mod_b;
                // next_mod = 10 ^ (2*i) * p + q
                doubling[i][next_mod] =
                    (doubling[i][next_mod] + doubling[i - 1][p] * doubling[i - 1][q]) % MOD;
            }
        }
    }
}

fn get_result(n: usize, doubling: &Vec<Vec<usize>>, power_of_10: &[usize]) -> Vec<usize> {
    let max_log = doubling.len();
    let mod_b = doubling[0].len();
    let mut result: Vec<usize> = vec![0; mod_b];
    result[0] = 1;

    for i in 0..max_log {
        if (n >> i) & 1 == 1 {
            let mut next_result: Vec<usize> = vec![0; mod_b];
            for p in 0..mod_b {
                if result[p] == 0 {
                    continue;
                }
                for q in 0..mod_b {
                    let next_mod = (p * power_of_10[i] + q) % mod_b;
                    next_result[next_mod] =
                        (next_result[next_mod] + result[p] * doubling[i][q]) % MOD;
                }
            }
            result = next_result;
        }
    }
    result
}
