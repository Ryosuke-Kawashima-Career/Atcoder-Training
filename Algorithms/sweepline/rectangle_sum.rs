use proconio::input;
// Rectangle Sum
// Q. There are some points on the coordinate plane with weights.
// A. Sweepline algorithm by fixing X-axis and moving Y-axis.
// Range sum -> Fenwick Tree
// Coordinate compression -> Binary Search
struct FenwickTree {
    tree: Vec<u64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut idx: usize, val: u64) {
        idx += 1;
        while idx < self.tree.len() {
            self.tree[idx] += val;
            idx += idx & (1 << idx.trailing_zeros());
        }
    }

    fn sum(&self, mut idx: usize) -> u64 {
        let mut res = 0;
        while idx > 0 {
            res += self.tree[idx];
            idx -= idx & (1 << idx.trailing_zeros());
        }
        res
    }
}

struct Event {
    x: i64,
    q_idx: usize,
    d_idx: usize,
    u_idx: usize,
    coeff: i64,
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut points: [(i64, i64, u64); n],
        queries: [(i64, i64, i64, i64); q], // (l, d, r, u)
    }

    // Coordinate compression on Y coordinates
    let mut y_coords: Vec<i64> = points.iter().map(|p| p.1).collect();
    y_coords.sort();
    y_coords.dedup();

    // Generate events
    let mut events = Vec::with_capacity(2 * q);
    for (i, &(l, d, r, u)) in queries.iter().enumerate() {
        let d_idx = y_coords.partition_point(|&y| y < d);
        let u_idx = y_coords.partition_point(|&y| y < u);
        events.push(Event {
            x: l,
            q_idx: i,
            d_idx,
            u_idx,
            coeff: -1,
        });
        events.push(Event {
            x: r,
            q_idx: i,
            d_idx,
            u_idx,
            coeff: 1,
        });
    }

    // Sort points by X
    points.sort_by_key(|p| p.0);

    // Sort events by X
    events.sort_by_key(|e| e.x);

    let mut ans = vec![0i64; q];
    let mut bit = FenwickTree::new(y_coords.len());
    let mut pt_idx = 0;

    for e in events {
        while pt_idx < n && points[pt_idx].0 < e.x {
            let comp_y = y_coords.partition_point(|&y| y < points[pt_idx].1);
            bit.add(comp_y, points[pt_idx].2);
            pt_idx += 1;
        }
        let val = bit.sum(e.u_idx) - bit.sum(e.d_idx);
        ans[e.q_idx] += e.coeff * (val as i64);
    }

    for a in ans {
        println!("{}", a);
    }
}
