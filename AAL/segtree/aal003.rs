use proconio::input;

#[derive(Debug, Clone)]
struct SegmentTree {
    data: Vec<i64>,
    size: usize,
}

impl SegmentTree {
    fn new(a: &[i64]) -> Self {
        let n: usize = a.len();
        let size: usize = n.next_power_of_two();
        let mut data: Vec<i64> = vec![0; size * 2];
        for i in 0..n {
            data[size + i] = a[i];
        }
        for i in (1..size).rev() {
            data[i] = data[i * 2] + data[i * 2 + 1];
        }
        Self { data, size }
    }

    // Type 1: Change A[i] to x (point assignment, not addition).
    fn update(&mut self, mut i: usize, x: i64) {
        i += self.size;
        self.data[i] = x;
        while i > 1 {
            i /= 2;
            self.data[i] = self.data[i * 2] + self.data[i * 2 + 1];
        }
    }

    // Half-open range sum query on [l, r).
    fn query(&self, mut l: usize, mut r: usize) -> i64 {
        let mut result: i64 = 0;
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 == 1 {
                result += self.data[l];
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                result += self.data[r];
            }
            l /= 2;
            r /= 2;
        }
        result
    }

    // Finds the smallest 0-based index `idx` such that sum(A[0..=idx]) >= target.
    // Assumes target >= 1 and all A[i] >= 0.
    fn find_first_prefix(&self, mut target: i64) -> Option<usize> {
        if self.data[1] < target {
            return None;
        }
        let mut idx: usize = 1;
        while idx < self.size {
            let left: usize = idx * 2;
            if self.data[left] >= target {
                idx = left;
            } else {
                target -= self.data[left];
                idx = left + 1;
            }
        }
        Some(idx - self.size)
    }

    // Type 2: Smallest r in [l, n) such that sum(A[l..=r]) >= t.
    // Equivalent to sum(A[0..=r]) >= sum(A[0..l]) + t.
    fn binary_search_right(&self, l: usize, t: i64) -> Option<usize> {
        let prefix_before_l: i64 = self.query(0, l);
        self.find_first_prefix(prefix_before_l + t)
    }

    // Type 3: Largest l in [0, r] such that sum(A[l..=r]) >= t.
    // Equivalent to finding the smallest l where sum(A[0..=l]) >= (sum(A[0..=r]) - t) + 1.
    fn binary_search_left(&self, r: usize, t: i64) -> Option<usize> {
        let prefix_up_to_r: i64 = self.query(0, r + 1);
        if prefix_up_to_r < t {
            return None;
        }
        self.find_first_prefix(prefix_up_to_r - t + 1)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [(usize, usize, i64); q],
    }
    let mut segtree = SegmentTree::new(&a);
    for &(query_type, idx, val) in &queries {
        if query_type == 1 {
            let (i, x) = (idx - 1, val);
            segtree.update(i, x);
        } else if query_type == 2 {
            let (l, t) = (idx - 1, val);
            match segtree.binary_search_right(l, t) {
                Some(r) => println!("{}", r + 1),
                None => println!("{}", n + 1),
            }
        } else {
            let (r, t) = (idx - 1, val);
            match segtree.binary_search_left(r, t) {
                Some(l) => println!("{}", l + 1),
                None => println!("{}", 0),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_with_naive() {
        let mut a = vec![3, 0, 2, 5, 1, 4];
        let n = a.len();
        let mut segtree = SegmentTree::new(&a);

        let updates = [(1, 0, 2), (3, 10, 0), (1, 4, 1), (5, 0, 2)];
        for &(pos, new_val, _) in &updates {
            a[pos] = new_val;
            segtree.update(pos, new_val);

            for l in 0..n {
                for t in 1..=25 {
                    let mut expected_r = None;
                    let mut sum = 0;
                    for r in l..n {
                        sum += a[r];
                        if sum >= t {
                            expected_r = Some(r);
                            break;
                        }
                    }
                    assert_eq!(segtree.binary_search_right(l, t), expected_r);
                }
            }

            for r in 0..n {
                for t in 1..=25 {
                    let mut expected_l = None;
                    let mut sum = 0;
                    for l in (0..=r).rev() {
                        sum += a[l];
                        if sum >= t {
                            expected_l = Some(l);
                            break;
                        }
                    }
                    assert_eq!(segtree.binary_search_left(r, t), expected_l);
                }
            }
        }
    }
}
