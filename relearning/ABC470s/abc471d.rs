use proconio::input;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cell {
    time: i64,
    watt: i64,
}
impl Ord for Cell {
    fn cmp(&self, other: &Cell) -> std::cmp::Ordering {
        let self_value: i64 = self.watt - self.time;
        let other_value: i64 = other.watt - other.time;
        self_value.cmp(&other_value)
    }
}
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Cell) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn main() {
    input!{q: usize, v: i64}
    let mut max_heap = std::collections::BinaryHeap::new();
    for _query in 0..q {
        input!{query_type: usize}
        if query_type == 1 {
            input!{tq: i64, wq: i64}
            max_heap.push(Cell {time: tq, watt: wq});
        } else {
            input!{tq: i64}
            if let Some(cell) = max_heap.pop() {
                let charge: i64 = v.min(cell.watt + tq - cell.time);
                println!("{}", charge);
            } else {
                println!("-1");
            }
        }
    }
}
