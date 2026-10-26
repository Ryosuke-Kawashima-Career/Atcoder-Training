use proconio::input;
use std::cmp::Ordering;
// ABC481E
// Q. Divise the XY coordinate by a line going through the origin.
// A. Argsort and Two pointers algorithm
#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    /// Returns 0 for the upper half-plane (y > 0, or y == 0 and x > 0),
    /// representing angles in [0, pi).
    /// Returns 1 for the lower half-plane (y < 0, or y == 0 and x < 0),
    /// representing angles in [pi, 2*pi).
    fn half(&self) -> u8 {
        if self.y > 0 || (self.y == 0 && self.x > 0) {
            0
        } else {
            1
        }
    }

    /// 2D Cross product: self.x * other.y - self.y * other.x
    /// > 0 if `other` is counter-clockwise (strictly less than 180 degrees) from `self`
    /// < 0 if `other` is clockwise from `self`
    fn cross(&self, other: &Point) -> i128 {
        (self.x as i128) * (other.y as i128) - (self.y as i128) * (other.x as i128)
    }
}

/// Exact integer comparator for Argument Sort (Polar Angle Sort) in [0, 2*pi)
fn arg_cmp(a: &Point, b: &Point) -> Ordering {
    let ha = a.half();
    let hb = b.half();
    if ha != hb {
        return ha.cmp(&hb);
    }
    // Same half-plane: if a.cross(b) > 0, `a` has a smaller angle than `b`
    let c = a.cross(b);
    if c > 0 {
        Ordering::Less
    } else if c < 0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn solve() {
    input! {
        n: usize,
        pts: [(i64, i64); n],
    }

    let mut p: Vec<Point> = pts.into_iter().map(|(x, y)| Point { x, y }).collect();

    // Step 1: Argument sort (polar angle sort) around the origin in O(N log N)
    p.sort_unstable_by(arg_cmp);

    // Step 2: Duplicate the array to handle circular wrap-around
    let mut p2 = Vec::with_capacity(2 * n);
    p2.extend_from_slice(&p);
    p2.extend_from_slice(&p);

    // Step 3: Two pointers (sliding window) to find the maximum points in an open half-plane (< 180 degrees)
    let mut ans = 0;
    let mut r = 0;
    for l in 0..n {
        if r < l {
            r = l;
        }
        // Keep advancing `r` while p2[r] is within < 180 degrees CCW from p2[l]
        // Note: no two stars are collinear with the origin, so cross != 0 for distinct stars.
        while r < l + n && (r == l || p2[l].cross(&p2[r]) > 0) {
            r += 1;
        }
        ans = ans.max(r - l);
        // Also consider the opposite half-plane of the same dividing line: `n - (r - l)`
        ans = ans.max(n - (r - l));
    }

    println!("{}", ans);
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}
