use proconio::input;

fn main() {
    input! {n: usize, q: usize, mut a: [i64; n], xy: [(i64, usize); q]}
    a.sort();
    for &(x, y) in xy.iter() {
        let ans: i64 = solve(x, y, &a);
        println!("{}", ans);
    }
}

fn solve(x: i64, y: usize, a: &Vec<i64>) -> i64 {
    // binary search for the yth largest number among the numbers >= x
    let n: usize = a.len();
    let start_index: usize = a.partition_point(|&num| num < x);
    let mut ok: isize = n as isize;
    let mut ng: isize = start_index as isize - 1;
    while ok - ng > 1 {
        let mid: isize = (ok + ng) / 2;
        // value and index
        if a[mid as usize] - x + 1 - (mid as i64 - start_index as i64 + 1) >= y as i64 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans: i64 = x + (y as i64 - 1) + (ok as i64 - start_index as i64);
    ans
}
