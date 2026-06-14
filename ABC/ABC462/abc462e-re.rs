use proconio::input;
// abc462e
// Q. Move to neighboring cell: if the step is odd, cost of movement x is `A` and that of movement y is `B`. The even version has the opposite.
// A. If `x + y` is even, it is possible to reach it from the origin with only diagonal movements.
fn solve_for_even(a: i64, b: i64, x: i64, y: i64) -> i64 {
    /*
    Returns minimum cost to reach `(x, y)` from `(0, 0)` where `x + y` is even.
    Based on two categories of the movements: 1. Diagonal 2. Horizontal
    */
    let x_abs = x.abs();
    let y_abs = y.abs();
    let min_ab: i64 = a.min(b);
    let diff_xy: i64 = (y_abs - x_abs).abs();
    let min_movement: i64 = y_abs.min(x_abs);
    // Diagonal Movement
    let diagonal: i64 = min_ab * min_movement * 2;
    // Double Movement
    let double: i64 = diff_xy / 2 * (a + b).min(4 * min_ab);
    return diagonal + double;
}
fn solve_for_odd(a: i64, b: i64, x: i64, y: i64) -> i64 {
    let mut min_cost: i64 = i64::MAX;
    min_cost = min_cost.min(solve_for_even(a, b, x - 1, y) + a);
    min_cost = min_cost.min(solve_for_even(a, b, x + 1, y) + a);
    min_cost = min_cost.min(solve_for_even(a, b, x, y - 1) + b);
    min_cost = min_cost.min(solve_for_even(a, b, x, y + 1) + b);
    return min_cost;
}
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {a: i64, b: i64, x: i64, y: i64}
        let ans = if (x + y).rem_euclid(2) == 0 {
            solve_for_even(a, b, x, y)
        } else {
            solve_for_odd(a, b, x, y)
        };
        println!("{}", ans);
    }
}
