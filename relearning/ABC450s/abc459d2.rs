use proconio::{input, marker::Chars};

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {s: Chars}
        let result = solve(&s);
        let n: usize = result.len();
        if n == 0 {
            continue;
        }
        for i in 0..n {
            print!("{}", result[i]);
        }
        println!("");
    }
}

fn solve(s: &Vec<char>) -> Vec<char> {
    /* Greedy algorithm */
    let mut n: usize = s.len();
    let mut result: Vec<char> = Vec::new();
    let mut count: Vec<usize> = vec![0; 26];
    for i in 0..n {
        count[s[i] as usize - 'a' as usize] += 1;
        if count[s[i] as usize - 'a' as usize] > (n + 1) / 2 {
            println!("No");
            return vec![];
        }
    }
    let mut prev_alpha: usize = 26;
    for _ in 0..n {
        let mut max_count: usize = 0;
        let mut selected_alpha: usize = 26;
        for alpha in 0..26 {
            if prev_alpha == alpha {
                continue;
            }
            if count[alpha] > max_count {
                max_count = count[alpha];
                selected_alpha = alpha;
            }
        }
        if max_count > 0 {
            result.push((selected_alpha as u8 + 'a' as u8) as char);
            count[selected_alpha] -= 1;
            prev_alpha = selected_alpha;
        }
    }
    println!("Yes");
    return result;
}
