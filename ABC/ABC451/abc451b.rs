use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, m: usize, ab: [(Usize1, Usize1); n]}
    let mut count_before: Vec<isize> = vec![0; m];
    let mut count_after: Vec<isize> = vec![0; m];
    for (a, b) in ab {
        count_before[a] += 1;
        count_after[b] += 1;
    }
    for dept in 0..m {
        let ans = count_after[dept] - count_before[dept];
        println!("{}", ans);
    }
}
