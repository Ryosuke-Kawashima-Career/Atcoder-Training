use proconio::input;
// abc460b
// Q. Judge whether the given two circles have intersections
// A. The distance between the centers and Radiuses of the two
fn has_intersections(x1: i64, y1: i64, r1: i64, x2: i64, y2: i64, r2: i64) -> bool {
    let dx = x1 - x2;
    let dy = y1 - y2;
    let d2 = (dx as i128) * (dx as i128) + (dy as i128) * (dy as i128);
    
    let r_sum = (r1 + r2) as i128;
    let r_diff = (r1 - r2) as i128;
    
    let r_sum_sq = r_sum * r_sum;
    let r_diff_sq = r_diff * r_diff;
    
    r_diff_sq <= d2 && d2 <= r_sum_sq
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            x1: i64, y1: i64, r1: i64,
            x2: i64, y2: i64, r2: i64,
        }
        if has_intersections(x1, y1, r1, x2, y2, r2) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

