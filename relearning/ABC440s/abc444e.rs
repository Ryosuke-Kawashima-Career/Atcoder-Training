use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    // this is the answer list of digits
    // e.g. 111 = vec![1, 1, 1]
    let answer = solve(&a);
    let output: String = answer
        .iter()
        .rev()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", output);
}

fn solve(a: &[usize]) -> Vec<usize> {
    /* 1. Get the numbr of c[j] such that 10 ^ j is added to the answer
    2.
     */
    let mut digits: Vec<usize> = Vec::new();
    let n: usize = a.len();
    // freq[v] := the number of a[i] == v

    // count[v] := the number of a[i] >= v = contribution of 10 ^ v

    // add up digits

    return digits;
}
