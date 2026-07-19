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
        // A, B, C from Ax + By + C = 0
        let a1: i128 = 2 * (qx - px);
        let b1: i128 = 2 * (qy - py);
        let c1: i128 = (px * px - qx * qx) + (py * py - qy * qy);
        let a2: i128 = 2 * (sx - rx);
        let b2: i128 = 2 * (sy - ry);
        let c2: i128 = (rx * rx - sx * sx) + (ry * ry - sy * sy);

        if judge_parallel(a1, b1, c1, a2, b2, c2) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn judge_parallel(a1: i128, b1: i128, c1: i128, a2: i128, b2: i128, c2: i128) -> bool {
    let (a1_normalized, b1_normalized, c1_normalized) = normalize(a1, b1, c1);
    let (a2_normalized, b2_normalized, c2_normalized) = normalize(a2, b2, c2);

    if a1_normalized * b2_normalized - a2_normalized * b1_normalized == 0 {
        if a1_normalized * c2_normalized - a2_normalized * c1_normalized == 0
            && b1_normalized * c2_normalized - b2_normalized * c1_normalized == 0
        {
            return true;
        } else {
            return false;
        }
    } else {
        return true;
    }
}

fn normalize(a: i128, b: i128, c: i128) -> (i128, i128, i128) {
    let mut a_norm: i128 = a;
    let mut b_norm: i128 = b;
    let mut c_norm: i128 = c;
    if a < 0 {
        a_norm = -a_norm;
        b_norm = -b_norm;
        c_norm = -c_norm;
    }
    let gcd_abc: i128 = gcd(gcd(a, b), c);
    if gcd_abc != 0 {
        a_norm /= gcd_abc;
        b_norm /= gcd_abc;
        c_norm /= gcd_abc;
    }
    (a_norm, b_norm, c_norm)
}

fn gcd(n: i128, m: i128) -> i128 {
    if n == 0 || m == 0 {
        return n + m;
    }
    gcd(m, n % m)
}
