use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, a: [Usize1; n]}
    let mut answer: Vec<usize> = (0..n).collect();
    for i in (0..n).rev() {
        answer[i] = answer[a[i]];
    }
    for i in 0..n {
        print!("{} ", answer[i] + 1);
    }
    println!("");
}
