use proconio::input;
// ABC397D
// Q. How many pairs of (x, y) s.t. x^3 - y^3 = n, where x, y are positive integers?
// A. Factorization -> Binary Search
fn main() {
    input! {n: i64}
    let ans = brute_force_difference_of_cubes(n);
    if ans.0 == -1 {
        println!("-1");
    } else {
        println!("{} {}", ans.0, ans.1);
    }
}

fn brute_force_difference_of_cubes(n: i64) -> (i64, i64) {
    let mut d: i64 = 1;
    while d * d * d <= n {
        // (y+d)^3 - y^3 = d^3 + 3*d^2y + 3*d*y^2 = d(d^2 + 3dy + 3y^2) = n
        if (n % d) != 0 {
            d += 1;
            continue;
        }
        // m = d^2 + 3dy + 3y^2
        let m = n / d;
        // 3y^2 + 3dy + d^2 - m = 0
        let y: i64 = solve_quadratic_positive_integer(3, 3 * d, d * d - m);
        if y > 0 {
            let x = y + d;
            return (x, y);
        }
        d += 1;
    }
    (-1, -1)
}

fn solve_quadratic_positive_integer(a: i64, b: i64, c: i64) -> i64 {
    /* solves ax^2 + bx + c = 0 for positive x */
    let quad = |x: i64| a * x * x + b * x + c;
    let mut xl: i64 = 0;
    let mut xr: i64 = 2_000_000_000; // Sufficiently large (sqrt(10^18/3) approx 6e8) but safe from overflow
    while xr - xl > 1 {
        let x_mid: i64 = (xl + xr) / 2;
        if quad(x_mid) > 0 {
            xr = x_mid;
        } else {
            xl = x_mid;
        }
    }
    if quad(xl) == 0 {
        xl
    } else {
        -1
    }
}
