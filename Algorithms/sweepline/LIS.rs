use proconio::input;
// Longest Increasing Sequence
// chokudai speedrun 001 H
fn main() {
    input! {n: usize, a: [usize; n]}
    // tails[length] = min val of the sequence whose length is "length" and is increasing
    let mut tails: Vec<usize> = Vec::new();

    for i in 0..n {
        let index: usize = tails.binary_search(&a[i]).unwrap_or_else(|i| i);
        if index < tails.len() {
            tails[index] = a[i];
        } else {
            tails.push(a[i]);
        }
    }

    println!("{}", tails.len());
}
