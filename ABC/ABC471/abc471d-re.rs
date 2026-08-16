use proconio::input;
use std::cmp::Ordering;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cell {
    time: i64,
    watt: i64,
}
impl Cell {
    fn new(t: i64, w: i64) -> Cell {
        Cell { time: t, watt: w }
    }
}
impl Ord for Cell {
    fn cmp(&self, other: &Cell) -> Ordering {
        let self_diff: i64 = self.watt - self.time;
        let other_diff: i64 = other.watt - other.watt;
        self_diff.cmp(&other_diff)
    }
}
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Cell) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    input!{q: usize, v: i64}
    let mut cur_time: i64 = 0;
    let mut bh = std::collections::BinaryHeap::new();
    for _ in 0..q {
        input!{query_type: usize}
        if query_type == 1 {
            input!{tq: i64, wq: i64}
            bh.push(Cell::new(tq, wq));
        } else {
            input!{tq: i64}
            if let Some(top) = bh.pop() {
                let watt_ans: i64 = v.min(top.watt + tq - top.time);
                cur_time = tq;
                println!("{}", watt_ans);
            } else {
                println!("-1");
            }
        }
    }
}
