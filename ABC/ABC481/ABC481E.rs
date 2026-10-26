use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, mut xy: [(i64, i64); n]}
        xy.sort_by(|node1, node2| arg_cmp(node1, node2));
        let mut points: Vec<(i64, i64)> = Vec::new();
        for i in 0..2 * n {
            points.push((xy[i % n].0, xy[i % n].1));
        }
        let ans = two_pointers(points);
        println!("{}", ans);
    }
}

fn get_half(p: (i64, i64)) -> usize {
    let (x, y) = p;
    if y > 0 || (y == 0 && x > 0) {
        return 0;
    } else {
        return 1;
    }
}

fn cross(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    x1 * y2 - x2 * y1
}

fn arg_cmp(p1: &(i64, i64), p2: &(i64, i64)) -> Ordering {
    let half1: usize = get_half(*p1);
    let half2: usize = get_half(*p2);
    if half1 != half2 {
        return half1.cmp(&half2);
    }
    let c: i64 = cross(*p1, *p2);
    if c > 0 {
        return Ordering::Less;
    } else if c < 0 {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

fn two_pointers(points: Vec<(i64, i64)>) -> usize {
    let n: usize = points.len();
    let mut result: usize = 0;
    let mut right: usize = 0;
    for left in 0..n {
        if right < left {
            right = left;
        }
        while right < left + n && cross(points[left], points[right]) > 0 {
            right += 1;
        }
        result = result.max(right - left);
    }
    result
}
