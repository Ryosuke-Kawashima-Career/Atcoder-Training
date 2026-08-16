use proconio::input;
// ABC471C
// Q. Start from the $(0, 0)$ and visit all points $(a_i, a_i)$ with $a_i != 0$ and minimize the total distance traveled.
// A. Divide the coordinate by the criterion of negative and positive.
fn main() {
    input!{n: usize, a: [i64; n]}
    let mut negatives: Vec<i64> = a.iter().filter(|&&x| x < 0).copied().collect();
    let mut positives: Vec<i64> = a.iter().filter(|&&x| x > 0).copied().collect();
    negatives.sort();
    positives.sort_by(|x1, x2| x2.cmp(&x1));
    let mut cur_x: i64 = 0;
    let mut cost: i64 = 0;
    for _ in 0..n {
        match (negatives.last(), positives.last()) {
            (Some(&x_neg), Some(&x_pos)) => {
                if cur_x - x_neg <= x_pos - cur_x {
                    cost += cur_x - x_neg;
                    cur_x = x_neg;
                    negatives.pop();
                } else {
                    cost += x_pos - cur_x;
                    cur_x = x_pos;
                    positives.pop();
                }
            },
            (Some(&x_neg), None) => {
                cost += cur_x - x_neg;
                cur_x = x_neg;
                negatives.pop();
            },
            (None, Some(&x_pos)) => {
                cost += x_pos - cur_x;
                cur_x = x_pos;
                positives.pop();
            },
            (None, None) => break,
        }
    }
    println!("{}", cost);
}
