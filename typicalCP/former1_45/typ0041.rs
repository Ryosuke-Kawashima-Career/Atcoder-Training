use proconio::input;
// Typical CP 041
// Q. Get the number of lattice points in a polygon
// A. Outer product to calculate the Convex Hull
fn main() {
    input! { n: usize, xy: [(i64, i64); n] }
    let hull = get_outer_points(&xy);
    println!("{}", count_lattice_points(&hull) - n as i128);
}

type Point = (i64, i64);

// Positive for a counterclockwise turn, zero for collinear points.
fn cross(a: Point, b: Point, c: Point) -> i128 {
    (b.0 - a.0) as i128 * (c.1 - a.1) as i128 - (b.1 - a.1) as i128 * (c.0 - a.0) as i128
}

// Andrew's monotone chain: retain only the endpoints of straight edges.
fn get_outer_points(points: &[Point]) -> Vec<Point> {
    let mut points = points.to_vec();
    points.sort_unstable();

    let mut lower = Vec::new();
    for &point in &points {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], point) <= 0
        {
            lower.pop();
        }
        lower.push(point);
    }

    let mut upper = Vec::new();
    for &point in points.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], point) <= 0
        {
            upper.pop();
        }
        upper.push(point);
    }

    // Both chains include the leftmost and rightmost points.
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn count_lattice_points(hull: &[Point]) -> i128 {
    // A collinear input has a segment as its hull; Pick's theorem needs area > 0.
    if hull.len() == 2 {
        return gcd((hull[1].0 - hull[0].0).abs(), (hull[1].1 - hull[0].1).abs()) as i128 + 1;
    }

    let mut twice_area = 0_i128;
    let mut boundary = 0_i128;
    for i in 0..hull.len() {
        let (x1, y1) = hull[i];
        let (x2, y2) = hull[(i + 1) % hull.len()];
        twice_area += x1 as i128 * y2 as i128 - y1 as i128 * x2 as i128;
        boundary += gcd((x2 - x1).abs(), (y2 - y1).abs()) as i128;
    }

    // Pick: area = interior + boundary / 2 - 1.
    (twice_area.abs() + boundary) / 2 + 1
}
