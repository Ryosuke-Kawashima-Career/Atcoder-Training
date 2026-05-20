use proconio::input;
const MOD: usize = 998244353;
// Even if the person 0 is assigned to Team A, the generality is preserved.
fn main() {
    input! {n: usize, lr: [(i64, i64); n]}
    // dp[index][number of people in team A calculated by FENWICK]
    let mut dp: Vec<i64> = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..n {
        for num_a in 1..=i {}
    }
    let ans = dp[n - 1];
}

struct Fenwick {
    size: usize,
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            size,
            tree: vec![0; size + 1],
        }
    }
    fn add(&mut self, mut index: usize, value: i64) {
        index += 1;
        while index <= self.size {
            self.tree[index] += value;
            index += index & (!index + 1);
        }
    }
    fn query(&self, mut index: usize) -> i64 {
        index += 1;
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & (!index + 1);
        }
        sum
    }
}
