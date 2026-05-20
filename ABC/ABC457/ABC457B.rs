use proconio::input;

fn main() {
    input! {n: usize}
    let mut a: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {l: usize, al: [usize; l]}
        a.push(al);
    }
    input! {x: usize, y: usize}
    let ans = a[x - 1][y - 1];
    println!("{}", ans);
}
