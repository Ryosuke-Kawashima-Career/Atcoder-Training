use proconio::input;

fn main() {
    input! {n: usize, m: usize}
    let mut count: Vec<usize> = vec![0; n];
    let divident: usize = m / n;
    let remain: usize = m % n;
    for i in 0..n {
        count[i] = divident;
    }
    for i in 0..remain {
        count[i] += 1;
    }
    for i in 0..n {
        println!("{}", count[i]);
    }
}
