use proconio::input;
const MOD: usize = 998244353;
fn main() {
    input! {n: usize}
    let mut ans: usize = 0;
    for num in 1..=n {
        let count_conditions: usize =
            condition1(num) as usize + condition2(num) as usize + condition3(num) as usize;
        if count_conditions == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn condition1(num: usize) -> bool {
    num % 3 == 0
}

fn condition2(num: usize) -> bool {
    let num_chars: Vec<char> = num.to_string().chars().collect();
    for c in num_chars {
        if c == '3' {
            return true;
        }
    }
    return false;
}

fn condition3(num: usize) -> bool {
    let mut count = std::collections::HashSet::new();
    let num_chars: Vec<char> = num.to_string().chars().collect();
    for c in num_chars {
        count.insert(c);
    }
    count.len() == 3
}
