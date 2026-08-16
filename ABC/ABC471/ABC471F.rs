use proconio::input;

fn main() {
    input!{n: usize, k: usize, s: [String; n]}
    let mut numbers: Vec<Vec<String>> = vec![vec![]; 6];
    for i in 0..n {
        numbers[s[i].len()].push(s[i]);
    }
}
