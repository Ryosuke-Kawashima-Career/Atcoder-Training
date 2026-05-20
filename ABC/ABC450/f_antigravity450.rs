use proconio::input;

const MOD: u64 = 998244353;
const INV2: u64 = 499122177;

// A Lazy Segment Tree supporting Range Multiplication and Range Sum
struct SegTree {
    sum: Vec<u64>,
    lazy: Vec<u64>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        Self {
            sum: vec![0; size * 2],
            lazy: vec![1; size * 2],
        }
    }

    fn push_down(&mut self, node: usize) {
        let lz = self.lazy[node];
        if lz != 1 {
            self.lazy[node * 2] = (self.lazy[node * 2] * lz) % MOD;
            self.sum[node * 2] = (self.sum[node * 2] * lz) % MOD;
            
            self.lazy[node * 2 + 1] = (self.lazy[node * 2 + 1] * lz) % MOD;
            self.sum[node * 2 + 1] = (self.sum[node * 2 + 1] * lz) % MOD;
            
            self.lazy[node] = 1;
        }
    }

    fn push_up(&mut self, node: usize) {
        self.sum[node] = (self.sum[node * 2] + self.sum[node * 2 + 1]) % MOD;
    }

    fn mult(&mut self, ql: usize, qr: usize, v: u64, node: usize, l: usize, r: usize) {
        if ql <= l && r <= qr {
            self.lazy[node] = (self.lazy[node] * v) % MOD;
            self.sum[node] = (self.sum[node] * v) % MOD;
            return;
        }
        self.push_down(node);
        let mid = l + (r - l) / 2;
        if ql <= mid {
            self.mult(ql, qr, v, node * 2, l, mid);
        }
        if qr > mid {
            self.mult(ql, qr, v, node * 2 + 1, mid + 1, r);
        }
        self.push_up(node);
    }

    fn query(&mut self, ql: usize, qr: usize, node: usize, l: usize, r: usize) -> u64 {
        if ql > qr { return 0; }
        if ql <= l && r <= qr {
            return self.sum[node];
        }
        self.push_down(node);
        let mid = l + (r - l) / 2;
        let mut res = 0;
        if ql <= mid {
            res = (res + self.query(ql, qr, node * 2, l, mid)) % MOD;
        }
        if qr > mid {
            res = (res + self.query(ql, qr, node * 2 + 1, mid + 1, r)) % MOD;
        }
        res
    }

    fn set(&mut self, idx: usize, val: u64, node: usize, l: usize, r: usize) {
        if l == r {
            self.sum[node] = val;
            return;
        }
        self.push_down(node);
        let mid = l + (r - l) / 2;
        if idx <= mid {
            self.set(idx, val, node * 2, l, mid);
        } else {
            self.set(idx, val, node * 2 + 1, mid + 1, r);
        }
        self.push_up(node);
    }
}

fn power(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1;
    base %= MOD;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % MOD;
        }
        base = (base * base) % MOD;
        exp /= 2;
    }
    res
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    // Graph the incoming edges grouped by `Y` destination
    let mut ends_at = vec![vec![]; n + 1];
    for (x, y) in edges {
        ends_at[y].push(x);
    }

    let mut tree = SegTree::new(n);
    let size = n.next_power_of_two();

    // Base state probability: DP[1] = 1.0 (empty boundary)
    tree.set(1, 1, 1, 1, size);

    let mut dp_prev = 1;
    for i in 2..=n {
        for &x in &ends_at[i] {
            // Cut probability update: Multiplying [x, i-1] by 1/2
            if x <= i - 1 {
                tree.mult(x, i - 1, INV2, 1, 1, size);
            }
        }

        // DP[i] = 1 - sum(DP[1...i-1] * P(invalid cut))
        let sum_a = tree.query(1, i - 1, 1, 1, size);
        let dp_curr = (1 + MOD - sum_a) % MOD;
        
        tree.set(i, dp_curr, 1, 1, size);
        dp_prev = dp_curr;
    }

    // Final combinations: Expand probability out dynamically to 2^M
    let ans = (dp_prev * power(2, m as u64)) % MOD;
    println!("{}", ans);
}
