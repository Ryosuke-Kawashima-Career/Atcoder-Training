use proconio::input;

fn main() {
    input!{k: usize, a: [usize; k]}
    let mut does_9_exist: bool = false;
    let mut min_ball: usize = 10;
    for i in 0..k {
        if a[i] == 9 {
            does_9_exist = true;
        }
        min_ball = min_ball.min(a[i]);
    }
    if !does_9_exist {
        println!("-1");
    } else {
        println!("{}", min_ball);
    }
}
