use proconio::input;

fn main() {
    input!{t: usize}
    for _case in 0..t {
        input!{x: usize, y: usize, k: usize}
        let ans: usize = solve(x, y, k);
        println!("{}", ans);
    }
}

fn solve(mut x: usize, mut y: usize, k: usize) -> usize {
    let mut operations: usize = 0;
    while x != y {
        if x >= y {
            x /= k;
        } else {
            y /= k;
        }
        operations += 1;
    }
    return operations;
}
