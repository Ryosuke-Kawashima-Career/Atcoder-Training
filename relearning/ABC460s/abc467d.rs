use proconio::input;
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {px: i64, py: i64, qx: i64, qy: i64, rx: i64, ry: i64, sx: i64, sy: i64}
        let a1: i64 = 2 * (qx - px);
        let b1: i64 = 2 * (qy - py);
        let c1: i64 = px * px + py * py - qx * qx - qy * qy;
        let a2: i64 = 2 * (sx - rx);
        let b2: i64 = 2 * (sy - ry);
        let c2: i64 = rx * rx + ry * ry - sx * sx - sy * sy;

        if a1 * b2 - a2 * b1 == 0 {
            if is_same(a1, b1, c1, a2, b2, c2) {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("Yes");
        }
    }
}

fn is_same(a1: i64, b1: i64, c1: i64, a2: i64, b2: i64, c2: i64) -> bool {
    let (a1_norm, b1_norm, c1_norm) = normalize(a1, b1, c1);
    let (a2_norm, b2_norm, c2_norm) = normalize(a2, b2, c2);

    if a1_norm != a2_norm || b1_norm != b2_norm || c1_norm != c2_norm {
        return false;
    }

    return true;
}

fn normalize(mut a: i64, mut b: i64, mut c: i64) -> (i64, i64, i64) {
    if a < 0 {
        a = -a;
        b = -b;
        c = -c;
    }

    let g = gcd(a, gcd(b, c));
    a /= g;
    b /= g;
    c /= g;

    (a, b, c)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
