use proconio::input;
// abc467d
// Q. Judge if there are two circiles such that one circle includes P and Q while the other includes R and S.
// A. Check the perpendicular bisectors of segments PQ, RS. Let them be L1, L2.
//    If L1 and L2 are the same, then it is possible.
fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {
            px: i128, py: i128,
            qx: i128, qy: i128,
            rx: i128, ry: i128,
            sx: i128, sy: i128,
        }
        if (qx - px) * (sy - ry) - (qy - py) * (sx - rx) == 0 {
            if (sx - rx) * (px + qx - rx - sx) + (sy - ry) * (py + qy - ry - sy) == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("Yes");
        }
    }
}
