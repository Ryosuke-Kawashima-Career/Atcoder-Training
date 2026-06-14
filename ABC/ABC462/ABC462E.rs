use proconio::input;

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {a: i64, b: i64, x: i64, y: i64}
        let x_moves: i64 = x.abs();
        let y_moves: i64 = y.abs();
        let odd_moves: i64 = (x_moves + y_moves + 1) / 2;
        let even_moves: i64 = (x_moves + y_moves) / 2;
        let mut min_cost: i64 = i64::MAX;
        let mut candidates_of_odd_moves_x: Vec<i64> = Vec::new();
        if x_moves > even_moves {
            candidates_of_odd_moves_x.push(x_moves - even_moves);
        } else {
            candidates_of_odd_moves_x.push(0);
        }
        if x_moves > odd_moves {
            candidates_of_odd_moves_x.push(x_moves - odd_moves);
        } else {
            candidates_of_odd_moves_x.push(0);
        }
        for odd_moves_x in candidates_of_odd_moves_x {
            let even_moves_x = (x_moves - odd_moves_x).max(0);
            let odd_moves_y = (odd_moves - odd_moves_x).max(0);
            let even_moves_y = (even_moves - even_moves_x).max(0);
            let cost: i64 = a * (odd_moves_x + even_moves_y) + b * (even_moves_x + odd_moves_y);
            min_cost = min_cost.min(cost);
        }
        println!("{}", min_cost);
    }
}
