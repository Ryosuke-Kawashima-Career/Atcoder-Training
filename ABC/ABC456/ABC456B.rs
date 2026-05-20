use proconio::input;

fn main() {
    input! {dices: [[usize; 6]; 3]}
    let mut dices_456: Vec<Vec<usize>> = vec![vec![0; 3]; 3];
    for dice in 0..3 {
        for surface in 0..6 {
            if dices[dice][surface] >= 4 && dices[dice][surface] <= 6 {
                dices_456[dice][dices[dice][surface] - 4] += 1;
            }
        }
    }
    let target_combination: usize = dices_456[0][0] * dices_456[1][1] * dices_456[2][2]
        + dices_456[0][0] * dices_456[1][2] * dices_456[2][1]
        + dices_456[0][1] * dices_456[1][0] * dices_456[2][2]
        + dices_456[0][1] * dices_456[1][2] * dices_456[2][0]
        + dices_456[0][2] * dices_456[1][0] * dices_456[2][1]
        + dices_456[0][2] * dices_456[1][1] * dices_456[2][0];
    let ans = (target_combination as f64) / (6.0 * 6.0 * 6.0);
    println!("{}", ans);
}
