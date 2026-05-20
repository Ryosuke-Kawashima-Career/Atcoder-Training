use proconio::input;
// ABC397D
// Q. How many pairs of (x, y) s.t. x^3 - y^3 = n, where x, y are positive integers?
// A.
fn main() {
    input! {n: i64}
    let ans = get_cube_pair(n);
    if ans.0 == -1 {
        println!("-1");
    } else {
        println!("{} {}", ans.0, ans.1);
    }
}

fn get_cube_pair(n: i64) -> (i64, i64) {
    let mut y: i64 = 1;
    let mut x: i64 = 1;
    loop {
        let next_x3 = y * y * y + n;
        let next_x = cube_root(next_x3);
        if next_x * next_x * next_x == y * y * y + n {
            return (next_x, y);
        }
        x = next_x;
        let next_y3 = next_x * next_x * next_x - (next_x - 1) * (next_x - 1) * (next_x - 1);
        let next_y = cube_root(next_y3);
        if next_y * next_y * next_y + n == next_x * next_x * next_x {
            return (next_x, next_y);
        }
        if y == next_y {
            break;
        }
        y = next_y;
    }
    return (-1, -1);
}

fn cube_root(n: i64) -> i64 {
    let mut x = 1;
    loop {
        let x3 = x * x * x;
        if x3 == n {
            return x;
        }
        if x3 > n {
            return x - 1;
        }
        x += 1;
    }
    return x;
}
