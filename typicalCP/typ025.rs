use proconio::input;

fn main() {
    input!{n: usize, b: usize}
    let digits_b: usize = b.to_string().len();
    let mut dp: Vec<usize> = vec![0; digits_b + 1];
    
}

fn f(num: usize) -> usize {
    let digits: Vec<usize> = num.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut product: usize = 1;
    for digit in digits {
        product *= digit;
    }
    product
}
