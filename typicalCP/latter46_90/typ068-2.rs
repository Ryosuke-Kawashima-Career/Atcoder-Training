use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, Usize1, Usize1, i64); q],
    }

    // Store edge values b[k] = A[k] + A[k + 1] for k in 0..(n - 1)
    let mut segtree = Segtree::new(n.saturating_sub(1).max(1));

    for &(query_type, x, y, value) in &queries {
        if query_type == 0 {
            // T = 0: Learn A[x] + A[x + 1] = value
            segtree.set(x, value);
        } else {
            // T = 1: Given A[x] = value, find A[y]
            let l = x.min(y);
            let r = x.max(y);
            let res = segtree.query(l, r);
            let dist = r - l;

            if res.length == dist {
                // Telescoping identity:
                // res.value = b[l] - b[l+1] + ... + (-1)^(dist-1) * b[r-1]
                //           = A[l] + (-1)^(dist-1) * A[r]
                let ans = if dist % 2 == 1 {
                    // A[l] + A[r] = res.value
                    res.value - value
                } else if x < y {
                    // A[l] - A[r] = res.value, given A[l] = value -> A[r] = value - res.value
                    value - res.value
                } else {
                    // A[l] - A[r] = res.value, given A[r] = value -> A[l] = value + res.value
                    value + res.value
                };
                println!("{}", ans);
            } else {
                println!("Ambiguous");
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Monoid {
    // Alternating sum: b[0] - b[1] + b[2] - ... + (-1)^(length-1) * b[length-1]
    value: i64,
    // Number of known edges in this segment
    length: usize,
}

impl Monoid {
    fn identity() -> Self {
        Self {
            value: 0,
            length: 0,
        }
    }

    fn merge(left: Self, right: Self) -> Self {
        // The first element of `right` sits at index `left.length`,
        // so its sign is (-1)^(left.length).
        let signed_right = if left.length % 2 == 0 {
            right.value
        } else {
            -right.value
        };
        Self {
            value: left.value + signed_right,
            length: left.length + right.length,
        }
    }
}

struct Segtree {
    size: usize,
    data: Vec<Monoid>,
}

impl Segtree {
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let data = vec![Monoid::identity(); 2 * size];
        Self { size, data }
    }

    fn set(&mut self, index: usize, value: i64) {
        let mut cur = index + self.size;
        self.data[cur] = Monoid { value, length: 1 };
        while cur > 1 {
            cur /= 2;
            self.data[cur] = Monoid::merge(self.data[2 * cur], self.data[2 * cur + 1]);
        }
    }

    fn query(&self, index_l: usize, index_r: usize) -> Monoid {
        self._query(1, index_l, index_r, 0, self.size)
    }

    fn _query(
        &self,
        cell_index: usize,
        index_l: usize,
        index_r: usize,
        cell_left: usize,
        cell_right: usize,
    ) -> Monoid {
        if cell_right <= index_l || index_r <= cell_left {
            return Monoid::identity();
        }
        if index_l <= cell_left && cell_right <= index_r {
            return self.data[cell_index];
        }
        let child_index = cell_index * 2;
        let cell_mid = (cell_left + cell_right) / 2;
        let left_value = self._query(child_index, index_l, index_r, cell_left, cell_mid);
        let right_value = self._query(child_index + 1, index_l, index_r, cell_mid, cell_right);
        Monoid::merge(left_value, right_value)
    }
}
