use proconio::input;
// ABC474D
// Q. Find a `w` such that a * w > b * w
// A. Go to the extreme: Greedy Algorithm
fn main() {
    input! {n: usize, a: [i64; n], b: [i64; n]}
    let diff_a_from_b: Vec<i64> = (0..n).map(|i| a[i] - b[i]).collect();
    let a_superior_indexes: Vec<usize> = (0..n).filter(|&i| diff_a_from_b[i] > 0).collect();
    let sum_b: i64 = b.iter().sum();
    if a_superior_indexes.is_empty() {
        println!("No");
        return;
    }
    let min_diff_a_from_b = diff_a_from_b[a_superior_indexes[0]];
    let mut w: Vec<i64> = vec![1; n];
    w[a_superior_indexes[0]] = sum_b + 1;
    println!("Yes");
    for i in 0..n {
        print!("{} ", w[i]);
    }
    println!("");
}
