use proconio::input;

fn main() {
    input! {a: usize, b: usize}
    let ans: usize = num_to_digit(a + b);
    println!("{}", ans);
}

fn num_to_digit(num: usize) -> usize {
    let mut exp: usize = 1;
    let mut power: usize = 1;
    while power <= num {
        power *= 10;
        exp += 1;
    }
    exp - 1
}
