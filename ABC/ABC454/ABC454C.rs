use proconio::{input, marker::Usize1};

fn main() {
    // n: num of items m: people ab: Exchange itemA to itemB
    // Takahashi initially has Item0.
    input! {n: usize, m: usize, ab: [(Usize1, Usize1); m]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(item_a, item_b) in ab.iter() {
        graph[item_a].push(item_b);
    }
    let mut reachable: Vec<bool> = vec![false; n];
    let mut stack: Vec<usize> = vec![0];
    reachable[0] = true;
    while let Some(item) = stack.pop() {
        for &next_item in graph[item].iter() {
            if !reachable[next_item] {
                reachable[next_item] = true;
                stack.push(next_item);
            }
        }
    }
    let mut count: usize = 0;
    for i in 0..n {
        if reachable[i] {
            count += 1;
        }
    }
    println!("{}", count);
}
