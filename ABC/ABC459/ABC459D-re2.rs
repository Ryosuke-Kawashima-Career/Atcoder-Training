use proconio::{input, marker::Chars};
// ABC459 D: No adjacent cells have the same character
// Q. Given a string, find the minimum number of characters to change so that no two
// A. Greedy Algorith: Set the priorities for indexes with the same parity.
fn char_to_index(c: char) -> usize {
    return (c as u8 - b'a') as usize;
}
fn main() {
    input!{t: usize}
    for _case in 0..t {
        input!{s: Chars}
        solve(&s);
    }
}

fn solve(s: &Vec<char>) {
    let n: usize = s.len();
    let mut counts: Vec<usize> = vec![0; 26];
    let mut result: Vec<char> = vec![' '; n];

    for &c in s.iter() {
        counts[char_to_index(c)] += 1;
    }
    let max_count: usize = *counts.iter().max().unwrap();
    if max_count > (n + 1) / 2 {
        println!("No");
        return;
    }
    // parity
    let mut index_priority: Vec<usize> = Vec::new();
    for i in (0..n).step_by(2) {
        index_priority.push(i);
    }
    for i in (1..n).step_by(2) {
        index_priority.push(i);
    }
    index_priority.reverse();
    // argsort by counts
    let mut indexes_alpha: Vec<usize> = (0..26).collect();
    indexes_alpha.sort_by(|i1, i2| counts[*i2].cmp(&counts[*i1]));
    for &index_alpha in indexes_alpha.iter() {
        let c: char = (b'a' + index_alpha as u8) as char;
        for _ in 0..counts[index_alpha] {
            let index: usize = index_priority.pop().unwrap();
            result[index] = c;
        }
    }
    println!("Yes");
    println!("{}", result.iter().collect::<String>());
}