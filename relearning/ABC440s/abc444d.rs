use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    // this is the answer list of digits
    // e.g. 111 = vec![1, 1, 1]
    let answer = solve(&a);
    let output: String = answer
        .iter()
        .rev()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", output);
}

fn solve(a: &[usize]) -> Vec<usize> {
    let mut digits: Vec<usize> = Vec::new();
    let n: usize = a.len();
    for i in 0..n {
        let mut to_next: usize = 0;
        for digit in 0..a[i] {
            let mut current_digit: usize = if digits.len() <= digit {
                0
            } else {
                digits[digit]
            };
            current_digit += 1;
            current_digit += to_next;
            to_next = current_digit / 10;
            current_digit %= 10;
            if digits.len() <= digit {
                digits.push(current_digit);
            } else {
                digits[digit] = current_digit;
            }
        }
    }
    return digits;
}
