use proconio::input;

fn solve_even(x: i64, y: i64, a: i64, b: i64) -> i64 {
    let x = x.abs();
    let y = y.abs();
    let min_xy = std::cmp::min(x, y);
    let diff = (x - y).abs();
    let min_ab = std::cmp::min(a, b);

    // min_xy steps of diagonal (1, 1) cost 2 * min_ab each.
    // The remaining diff steps of (1, 0) are grouped into pairs of (2, 0).
    // A pair of (2, 0) costs min(a + b, 4 * min_ab).
    min_xy * 2 * min_ab + (diff / 2) * std::cmp::min(a + b, 4 * min_ab)
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            a: i64,
            b: i64,
            x: i64,
            y: i64,
        }

        if (x + y).rem_euclid(2) == 0 {
            println!("{}", solve_even(x, y, a, b));
        } else {
            // The shortest path must end with a single step from one of the 4 neighbors of (x, y).
            // Since (x, y) has an odd sum, its neighbors have an even sum.
            // A step from an even cell to an odd cell costs:
            // - A if the step is horizontal
            // - B if the step is vertical
            let mut ans = i64::MAX;

            // Neighbors via horizontal step (x +/- 1, y): last step cost is A
            ans = std::cmp::min(ans, solve_even(x + 1, y, a, b) + a);
            ans = std::cmp::min(ans, solve_even(x - 1, y, a, b) + a);

            // Neighbors via vertical step (x, y +/- 1): last step cost is B
            ans = std::cmp::min(ans, solve_even(x, y + 1, a, b) + b);
            ans = std::cmp::min(ans, solve_even(x, y - 1, a, b) + b);

            println!("{}", ans);
        }
    }
}
