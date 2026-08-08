use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let odd_palindromes = get_odd(&s);
    let even_palindromes = get_even(&s);
    let ans = odd_palindromes + even_palindromes;
    println!("{}", ans);
}

fn get_odd(s: &[char]) -> usize {
    let n = s.len();
    let mut odd_palindromes = 0;

    for center in 0..n {
        let mut radius = 0;
        let mut count = 0;
        while center >= radius && center + radius < n {
            if s[center - radius] != s[center + radius] {
                count += 1;
                if count > 1 {
                    break;
                }
            }
            radius += 1;
        }
        odd_palindromes += radius;
    }
    odd_palindromes
}

fn get_even(s: &[char]) -> usize {
    let n = s.len();
    let mut even_palindromes = 0;

    for center in 0..n.saturating_sub(1) {
        let mut radius = 0;
        let mut count = 0;
        while center >= radius && center + 1 + radius < n {
            if s[center - radius] != s[center + 1 + radius] {
                count += 1;
                if count > 1 {
                    break;
                }
            }
            radius += 1;
        }
        even_palindromes += radius;
    }
    even_palindromes
}
