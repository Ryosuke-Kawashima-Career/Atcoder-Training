use proconio::input;
use std::collections::BTreeSet;
fn main() {
    input! {x: usize, q: usize, ab: [(usize, usize); q]}
    let mut low: BTreeSet<usize> = BTreeSet::new();
    let mut high: BTreeSet<usize> = BTreeSet::new();
    low.insert(x);
    let mut cur_med: usize = x;

    for query in 0..q {
        let (num1, num2) = ab[query];
        for &num in &[num1, num2] {
            if num > cur_med {
                high.insert(num);
            } else {
                low.insert(num);
            }
        }
        while low.len() > high.len() + 1 {
            let max_of_low: usize = *low.last().unwrap();
            high.insert(max_of_low);
            low.pop_last();
        }
        while high.len() >= low.len() {
            let min_of_high: usize = high.pop_first().unwrap();
            low.insert(min_of_high);
        }
        let median: usize = *low.last().unwrap();
        cur_med = median;
        println!("{}", median);
    }
}
