use proconio::input;
// Implement a Fenwick tree
struct Fenwick {
    data: Vec<i64>,
}
impl Fenwick {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & (x.wrapping_neg())
    }
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size + 1],
        }
    }
    fn add(&mut self, mut x: usize, val: i64) {
        while x < self.data.len() {
            self.data[x] += val;
            x += Self::lsb(x);
        }
    }
    fn query(&self, mut x: usize) -> i64 {
        let mut sum = 0;
        while x > 0 {
            sum += self.data[x];
            x -= Self::lsb(x);
        }
        sum
    }
}
// Implement an Event struct of x, query_idx, y_indexes, coefficient: +1 or -1
#[derive(Debug)]
struct Event {
    x: i64,
    query_idx: usize,
    y_down_idx: usize,
    y_up_idx: usize,
    coefficient: i64,
}
impl Event {
    fn new(x: i64, query_idx: usize, y_down_idx: usize, y_up_idx: usize, coefficient: i64) -> Self {
        Self {
            x,
            query_idx,
            y_down_idx,
            y_up_idx,
            coefficient,
        }
    }
}
fn main() {
    input! {n: usize, q: usize, mut xyw: [(i64, i64, i64); n], queries: [(i64, i64, i64, i64); q]}
    // coord compression for the Y axis
    let mut y_coords: Vec<i64> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    for i in 0..n {
        y_coords.push(xyw[i].1);
    }
    y_coords.sort();
    y_coords.dedup();

    // Register Events for x = l and x = r
    for q_idx in 0..q {
        let (x_left, y_down, x_right, y_up) = queries[q_idx];
        let y_down_idx = y_coords.partition_point(|y| y < &y_down);
        let y_up_idx = y_coords.partition_point(|y| y < &y_up);

        // x < x_left: not yet added => coefficient -1
        events.push(Event::new(x_left, q_idx, y_down_idx, y_up_idx, -1));
        // x < x_right: remove it => coefficient +1
        events.push(Event::new(x_right, q_idx, y_down_idx, y_up_idx, 1));
    }

    // Sort points and events by x
    xyw.sort();
    events.sort_by(|e1, e2| e1.x.cmp(&e2.x));

    // Sweepline
    let mut ans: Vec<i64> = vec![0; q];
    let mut bit = Fenwick::new(y_coords.len());
    let mut point_index: usize = 0;
    for event in events {
        while point_index < n && xyw[point_index].0 < event.x {
            let comp_y_idx: usize = y_coords.partition_point(|y| y < &xyw[point_index].1);
            bit.add(comp_y_idx, xyw[point_index].2);
            point_index += 1;
        }
        let weight_sum: i64 = bit.query(event.y_up_idx) - bit.query(event.y_down_idx);
        ans[event.query_idx] += event.coefficient * weight_sum;
    }
    for i in 0..q {
        println!("{}", ans[i]);
    }
}
