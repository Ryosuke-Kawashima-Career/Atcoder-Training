use proconio::input;
// ABC397D
// Q. How many pairs of (x, y) s.t. x^3 - y^3 = n, where x, y are positive integers?
// A. Factorization -> Binary Search
fn main() {
    input! {n: i64}
    let ans = get_cube_pair(n);
    if ans.0 == -1 {
        println!("-1");
    } else {
        println!("{} {}", ans.0, ans.1);
    }
}

fn get_cube_pair(n: i64) -> (i64, i64) {
    // x^3 - y^3 = (x-y)(x^2 + xy + y^2) = N
    // Let k = x - y. Then k must divide N.
    // Also N = k(3y^2 + 3ky + k^2) > k^3 (since y >= 1)
    // So k ranges from 1 to approx N^(1/3).
    // For N = 10^18, k <= 10^6. This is fast enough.

    let mut k = 1;
    let n_u128 = n as u128;
    // Bound check: k^3 < 4N is safe.
    // In u128:
    while (k as u128) * (k as u128) * (k as u128) < 4 * n_u128 {
        // slightly loose bound is fine, actual bound is roughly (4N)^(1/3) derived from discriminant >= 0
        if n % k == 0 {
            // We need to solve 3y^2 + 3ky + (k^2 - N/k) = 0 for y.
            // Quadratic formula: y = (-3k + sqrt(D)) / 6
            // Discriminant D = (3k)^2 - 4*3*(k^2 - N/k)
            // D = 9k^2 - 12k^2 + 12(N/k) = 12(N/k) - 3k^2

            // Solve 3y^2 + 3ky + (k^2 - N/k) = 0 for y in integers
            // Discriminant D = 12(N/k) - 3k^2
            // We use u128 to prevent overflow since 12*N can exceed i64::MAX
            let val = n_u128 / (k as u128);

            // Note: 12 * val might be around 1.2 * 10^19. i64::MAX is ~9e18.
            let term1 = 12 * val;
            let term2 = 3 * (k as u128) * (k as u128);

            if term1 >= term2 {
                let discriminant = term1 - term2;
                let sqrt_d = integer_sqrt(discriminant);
                if sqrt_d * sqrt_d == discriminant {
                    // Check if numerator (-3k + sqrt_d) is divisible by 6 and positive
                    // We need -3k + sqrt_d > 0 => sqrt_d > 3k
                    // And divisibility by 6.

                    // working with i64 for final result components is preferred if they fit,
                    // but intermediate sqrt_d can be larger than i64 (roughly 3*10^9).
                    // Actually sqrt(10^19) is 3*10^9, which fits in i64 easily.
                    let sqrt_d_i64 = sqrt_d as i64;
                    let num = sqrt_d_i64 - 3 * k;

                    if num > 0 && num % 6 == 0 {
                        let y = num / 6;
                        let x = y + k;
                        return (x, y);
                    }
                }
            }
        }
        k += 1;
    }

    (-1, -1)
}

fn integer_sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    // N <= 1.2 * 10^19. Sqrt <= 4 * 10^9.
    // Binary search for safety.
    let mut low = 1;
    let mut high = 4_000_000_000; // ample upper bound
    let mut ans = 1;
    while low <= high {
        let mid = (low + high) / 2;
        // Check for overflow before multiplication
        if mid > u128::MAX / mid {
            // mid * mid would overflow
            high = mid - 1;
            continue;
        }
        if mid * mid <= n {
            ans = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    ans
}
