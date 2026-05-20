use proconio::input;
// ABC363D
// Q. What is the nth Palindrome?
// A. x is N digits, there are 9 * 10^((N-1)/2) palindromes
fn main() {
    input! { n: u64 }
    let nth_palindrome: String = get_palindrome(n);
    println!("{}", nth_palindrome);
}

fn get_palindrome(mut n: u64) -> String {
    if n == 1 {
        return "0".to_string();
    }
    n -= 1;
    let mut total_length = 1;
    loop {
        let half_length = (total_length + 1) / 2;
        let count = 9 * 10u64.pow(half_length as u32 - 1);
        if n <= count {
            let mut s = (10u64.pow(half_length as u32 - 1) + n - 1).to_string();
            let mut res = s.chars().collect::<Vec<char>>();
            res.resize(total_length as usize, ' ');
            for i in half_length as usize..total_length as usize {
                res[i] = res[total_length as usize - 1 - i];
            }
            return res.into_iter().collect();
        } else {
            n -= count;
        }
        total_length += 1;
    }
}
