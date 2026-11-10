use proconio::input;
// Approach: Segment Tree DP

#[derive(Debug, Clone)]
struct SegmentTree {
    size: usize,
    data: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let size: usize = n.next_power_of_two();
        let data: Vec<i64> = vec![-1; 2 * size];
        Self { size, data }
    }

    fn get(&self, index: usize) -> i64 {
        self.data[index + self.size]
    }

    fn query(&self, mut left: usize, mut right: usize) -> i64 {
        left += self.size;
        right += self.size;
        let mut result: i64 = -1;
        while left < right {
            if left & 1 == 1 {
                result = result.max(self.data[left]);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                result = result.max(self.data[right]);
            }
            left >>= 1;
            right >>= 1;
        }
        result
    }

    fn update(&mut self, mut index: usize, val: i64) {
        index += self.size;
        self.data[index] = val;
        while index > 1 {
            index >>= 1;
            self.data[index] = self.data[2 * index].max(self.data[2 * index + 1]);
        }
    }
}

fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize, usize, i64); n],
    }

    let mut dp: SegmentTree = SegmentTree::new(w + 1);
    dp.update(0, 0);

    for item in 0..n {
        let (l, r, v) = lrv[item];
        let mut new_dp: SegmentTree = dp.clone();
        for weight in l..=w {
            let curr_value: i64 = dp.get(weight);
            let weight_min: usize = weight.saturating_sub(r);
            let weight_max: usize = weight - l;
            let max_value_prev: i64 = dp.query(weight_min, weight_max + 1);
            if max_value_prev != -1 {
                let new_value: i64 = curr_value.max(max_value_prev + v);
                new_dp.update(weight, new_value);
            }
        }
        dp = new_dp;
    }

    let ans: i64 = dp.get(w);
    println!("{}", ans);
}
