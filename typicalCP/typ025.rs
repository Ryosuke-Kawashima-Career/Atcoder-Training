use proconio::input;

fn main() {
    input! { n: usize, b: usize }
    let mut ans: usize = 0;

    // Case 1: f(m) = 0 (m contains at least one '0')
    // Since m - 0 = b, the only candidate is m = b.
    if b <= n && b.to_string().contains('0') {
        ans += 1;
    }

    // Case 2: f(m) > 0 (m consists only of digits 1..=9)
    // num, next, len, n, b, ans
    dfs(0, 1, 0, n, b, &mut ans);
    println!("{}", ans);
}

fn f(num: usize) -> usize {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .product()
}

fn dfs(num: usize, next: usize, len: usize, n: usize, b: usize, ans: &mut usize) {
    if len > 0 {
        let rem: usize = f(num);
        let goal: usize = rem + b;
        if goal <= n {
            let mut goal_digits: Vec<char> = goal.to_string().chars().collect();
            goal_digits.sort_unstable();
            let num_digits: Vec<char> = num.to_string().chars().collect();
            if goal_digits == num_digits {
                *ans += 1;
            }
        }
    }

    if len == 11 {
        return;
    }

    for digit in next..=9 {
        let next_num = num * 10 + digit;
        dfs(next_num, digit, len + 1, n, b, ans);
    }
}

