use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m]
    }
    let mut interests: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in edges.iter() {
        interests[a].push(b);
        interests[b].push(a);
    }
    for i in 0..n {
        let interest: usize = interests[i].len();
        let non_interest: usize = n - 1 - interest;
        let ans: usize = if non_interest >= 3 {
            non_interest * (non_interest - 1) * (non_interest - 2) / 6
        } else {
            0
        };
        print!("{} ", ans);
    }
    println!("");
}
