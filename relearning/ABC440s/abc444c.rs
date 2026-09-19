use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();
    let mut answer: Vec<usize> = Vec::new();
    solve_case_intact(&a, &mut answer);
    solve_case_split(&a, &mut answer);
    let ans: String = answer
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}

fn solve_case_intact(a: &[usize], answer: &mut Vec<usize>) {
    let n: usize = a.len();
    let upper_bound: usize = a[n - 1];
    let candidate: usize = upper_bound;
    let mut left: usize = 0;
    let mut right: usize = n - 1;
    while right > 0 && a[right] == upper_bound {
        right -= 1;
    }
    while left + 1 < right {
        if a[left] + a[right] == candidate {
            left += 1;
            right -= 1;
        } else {
            return;
        }
    }
    answer.push(candidate);
}

fn solve_case_split(a: &[usize], answer: &mut Vec<usize>) {
    let n: usize = a.len();
    let upper_bound: usize = a[n - 1];
    let candidate: usize = a[0] + upper_bound;
    let mut left: usize = 0;
    let mut right: usize = n - 1;
    while right > 0 && a[right] == upper_bound {
        right -= 1;
    }
    while left + 1 < right {
        if a[left] + a[right] == candidate {
            left += 1;
            right -= 1;
        } else {
            return;
        }
    }
    answer.push(candidate);
}
