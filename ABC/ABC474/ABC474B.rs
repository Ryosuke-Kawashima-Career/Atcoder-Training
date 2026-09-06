use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, p: [Usize1; n]}
    for i in 0..n {
        if get_group(i) != get_group(p[i]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn get_group(n: usize) -> usize {
    n / 10
}
