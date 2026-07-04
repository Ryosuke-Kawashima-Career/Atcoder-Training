use proconio::input;

fn main() {
    input! {t: usize}
    for _case in 0..t {
        // change x to y1 or y2 s.t. y1 = x / k or y2 / k = x
        input! {x: usize, y: usize, k: usize}
        let ans = solve(x, y, k);
        println!("{}", ans);
    }
}

fn solve(mut x: usize, mut y: usize, k: usize) -> usize {
    let mut operations: usize = 0;
    loop {
        if x == y {
            return operations;
        } else if x > y {
            x /= k;
        } else {
            y /= k;
        }
        operations += 1;
    }
    return operations;
}
