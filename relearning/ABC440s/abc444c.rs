use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {n: usize, a: [usize; n]}
    let sum_a: usize = a.iter().sum();
    let mut answer: Vec<usize> = Vec::new();
    let a_set = a.iter().cloned().collect::<HashSet<_>>();
    divide(sum_a, &a_set, &mut answer);
    answer.sort();
    answer.dedup();
    println!(
        "{}",
        answer
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn divide(sum_a: usize, a_set: &HashSet<usize>, answer: &mut Vec<usize>) {
    /* Divide sum_a into elements contained in `a_set` */
    if sum_a == 0 {
        return;
    }
    for &element in a_set.iter() {
        if element > sum_a {
            continue;
        }
        let remainder = sum_a - element;
        answer.push(element);
        divide(remainder, a_set, answer);
        answer.pop();
    }
}
