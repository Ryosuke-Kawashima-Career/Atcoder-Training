use proconio::input;

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {a: i64, b: i64, x: i64, y: i64}
        let total_moves: i64 = x.abs() + b.abs();
        let mut min_cost: i64 = i64::MAX;
        if total_moves % 2 == 0 {
            min_cost = min_cost.min(solve(a, b, x, y));
        } else {
            min_cost = min_cost.min(solve(b, a, x - 1, y) + a);
            min_cost = min_cost.min(solve(b, a, x + 1, y) + a);
            min_cost = min_cost.min(solve(b, a, x, y - 1) + b);
            min_cost = min_cost.min(solve(b, a, x, y + 1) + b);
        }
        println!("{} ", min_cost);
    }
}

fn solve(a: i64, b: i64, x: i64, y: i64) -> i64 {
    let min_cost_move: i64 = a.min(b);
    let diagonal_moves: i64 = x.abs().min(y.abs());
    let double_moves: i64 = (x.abs() - y.abs()).abs() / 2;

    if y.abs() >= x.abs() {
        return 2 * min_cost_move * diagonal_moves + 2 * double_moves * b;
    } else {
        return 2 * min_cost_move * diagonal_moves + 2 * double_moves * a;
    }
}
