use proconio::input;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Cell {
    time: i64,
    watt: i64,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = self.watt - self.time;
        let other_value = other.watt - other.time;
        self_value.cmp(&other_value)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    input! {
        q: usize,
        v: i64,
    }
    let mut bh = BinaryHeap::new();
    let mut answers: Vec<i64> = Vec::new();

    for _ in 0..q {
        input! {
            query_type: usize,
        }
        if query_type == 1 {
            input! {
                tq: i64,
                wq: i64,
            }
            let cell = Cell { time: tq, watt: wq };
            bh.push(cell);
        } else {
            input! {
                tq: i64,
            }
            if let Some(top) = bh.pop() {
                let ans: i64 = v.min(tq + top.watt - top.time);
                answers.push(ans);
            } else {
                answers.push(-1);
            }
        }
    }

    for &a in &answers {
        println!("{}", a);
    }
}
