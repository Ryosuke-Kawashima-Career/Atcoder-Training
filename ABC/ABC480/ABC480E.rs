use proconio::input;

/// Checks if A_1 * (A_1 + d) * ... * (A_1 + (k - 1) * d) <= n without overflow.
fn check_product(a1: u128, d: u128, k: u128, n: u128) -> bool {
    let mut prod = 1u128;
    for i in 0..k {
        let term = a1 + i * d;
        prod = prod.saturating_mul(term);
        if prod > n {
            return false;
        }
    }
    true
}

/// Computes floor(sqrt(x)) exactly for u128.
fn isqrt(x: u128) -> u128 {
    if x == 0 {
        return 0;
    }
    let mut r = (x as f64).sqrt() as u128;
    // Correct any floating-point rounding inaccuracy
    while (r + 1) * (r + 1) <= x {
        r += 1;
    }
    while r * r > x {
        r -= 1;
    }
    r
}

fn main() {
    input! {
        n: u128,
    }

    let mut ans: u128 = 0;

    // Case 1: Sequences of length k = 3
    // A_1 * (A_1 + d) * (A_1 + 2d) <= N
    for a1 in 1u128.. {
        if a1 * (a1 + 1) * (a1 + 2) > n {
            break;
        }
        let m = n / a1;
        // (4d + 3*a1)^2 <= 8*m + a1^2
        let root = isqrt(8 * m + a1 * a1);
        let max_d = (root - 3 * a1) / 4;
        ans += max_d;
    }

    // Case 2: Sequences of length k >= 4
    // Here A_1 <= N^(1/4) <= 31622, and total valid (A_1, k) pairs is ~37,500
    for a1 in 1u128.. {
        if !check_product(a1, 1, 4, n) {
            break;
        }

        for k in 4u128.. {
            if !check_product(a1, 1, k, n) {
                break;
            }

            // Binary search for the maximum d >= 1 such that check_product(a1, d, k, n) is true
            let mut low = 1u128;
            let mut high = 600_000u128; // Since 1 * (1 + d) * (1 + 2d) * (1 + 3d) <= 10^18 => 6*d^3 < 10^18 => d < 550,322
            while low < high {
                let mid = low + (high - low + 1) / 2;
                if check_product(a1, mid, k, n) {
                    low = mid;
                } else {
                    high = mid - 1;
                }
            }
            ans += low;
        }
    }

    println!("{}", ans);
}
