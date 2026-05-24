use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, q: usize}
    let mut blocks: Vec<usize> = vec![0; n];
    let mut answers: Vec<usize> = Vec::new();
    let mut minimum: usize
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: Usize1}
            blocks[x] += 1;
        } else {
            input! {y: usize}
            let ans = solve(y, &blocks);
            answers.push(ans);
        }
    }
    for ans in answers {
        println!("{}", ans);
    }
}

fn solve(min_blocks: usize, blocks: &Vec<usize>) -> usize {
    let n: usize = blocks.len();
}
