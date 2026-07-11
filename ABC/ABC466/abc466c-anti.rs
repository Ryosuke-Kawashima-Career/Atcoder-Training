use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let n_str = read_line();
    if n_str.is_empty() {
        return;
    }
    let n: usize = n_str.parse().unwrap();

    let mut ans = 0;
    let mut r = 1;

    for i in 0..n {
        r = std::cmp::max(r, i + 1);
        while r < n {
            println!("? {} {}", i + 1, r + 1);
            io::stdout().flush().unwrap();
            let response = read_line();
            if response == "Yes" {
                r += 1;
            } else {
                break;
            }
        }
        ans += r - i - 1;
    }

    println!("! {}", ans);
    io::stdout().flush().unwrap();
}
