use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {n: usize}
    let mut count: HashMap<usize, usize> = HashMap::new();
    let mut x: usize = 1;
    while x * x <= n {
        let mut y: usize = x + 1;
        loop {
            if x * x + y * y <= n {
                *count.entry(x * x + y * y).or_insert(0) += 1;
                y += 1;
            } else {
                break;
            }
        }
        x += 1;
    }
    let mut answer: Vec<usize> = Vec::new();
    for (&k, &v) in count.iter() {
        if v == 1 {
            answer.push(k);
        }
    }
    answer.sort();
    println!("{}", answer.len());
    for &x in answer.iter() {
        print!("{} ", x);
    }
    println!("");
}
