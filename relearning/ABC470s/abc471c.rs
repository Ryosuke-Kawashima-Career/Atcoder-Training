use proconio::input;

fn main() {
    input!{n: usize, a: [i64; n]}
    let mut positives: Vec<i64> = a.iter().filter(|&&x| x > 0).copied().collect();
    positives.sort_by(|x1, x2| x2.cmp(&x1));
    let mut negatives: Vec<i64> = a.iter().filter(|&&x| x < 0).copied().collect();
    negatives.sort();
    let mut cur_pos: i64 = 0;
    let mut cost: i64 = 0;
    while positives.len() > 0 || negatives.len() > 0 {
        match (positives.last(), negatives.last()) {
            (Some(&x_pos), Some(&x_neg)) => {
                if x_pos - cur_pos >= cur_pos - x_neg {
                    cost += cur_pos - x_neg;
                    cur_pos = x_neg;
                    negatives.pop();
                } else {
                    cost += x_pos - cur_pos;
                    cur_pos = x_pos;
                    positives.pop();
                }
            },
            (Some(&x_pos), None) => {
                cost += x_pos - cur_pos;
                cur_pos = x_pos;
                positives.pop();
            },
            (None, Some(&x_neg)) => {
                cost += cur_pos - x_neg;
                cur_pos = x_neg;
                negatives.pop();
            },
            (None, None) => {break;}
        }
    }
    println!("{}", cost);
}