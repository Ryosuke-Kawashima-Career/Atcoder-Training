use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut neg: Vec<i64> = a.iter().filter(|&&x| x < 0).copied().collect();
    let mut pos: Vec<i64> = a.iter().filter(|&&x| x > 0).copied().collect();

    // neg sorted ascending: e.g. [-10, -5, -2], neg.pop() yields -2 (closest to 0)
    neg.sort();
    // pos sorted descending: e.g. [7, 4, 1], pos.pop() yields 1 (closest to 0)
    pos.sort_by(|x, y| y.cmp(x));

    let mut cur_pos: i64 = 0;
    let mut cost: i64 = 0;

    while !neg.is_empty() || !pos.is_empty() {
        match (neg.last(), pos.last()) {
            (Some(&l), Some(&r)) => {
                let dist_l = cur_pos - l;
                let dist_r = r - cur_pos;
                // If tied, pick the smaller coordinate (left)
                if dist_l <= dist_r {
                    cost += dist_l;
                    cur_pos = l;
                    neg.pop();
                } else {
                    cost += dist_r;
                    cur_pos = r;
                    pos.pop();
                }
            }
            (Some(&l), None) => {
                cost += cur_pos - l;
                cur_pos = l;
                neg.pop();
            }
            (None, Some(&r)) => {
                cost += r - cur_pos;
                cur_pos = r;
                pos.pop();
            }
            (None, None) => break,
        }
    }

    println!("{}", cost);
}
