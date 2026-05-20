use proconio::{input, marker::Usize1};
// ABC457E
// Q. 2 clothes are chosen. Judge wether we can make s[i]..=t[i] covered while the other part is not covered.
// A. Two edges Binary Search
// A. FenwickTree for counting the number of clothes that cover the interval [l, r].
struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut i: usize, val: i32) {
        i += 1;
        while i < self.tree.len() {
            self.tree[i] += val;
            i += i & (!i + 1);
        }
    }

    fn query(&self, mut i: usize) -> i32 {
        i += 1;
        let mut res = 0;
        while i > 0 {
            res += self.tree[i];
            i -= i & (!i + 1);
        }
        res
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, Usize1); m],
        q: usize,
        st: [(Usize1, Usize1); q],
    }

    let mut clothes_by_l = vec![vec![]; n];
    let mut clothes_by_r = vec![vec![]; n];
    for &(l, r) in &lr {
        clothes_by_l[l].push(r);
        clothes_by_r[r].push(l);
    }

    for i in 0..n {
        clothes_by_l[i].sort_unstable();
        clothes_by_r[i].sort_unstable();
    }

    let mut queries_by_s = vec![vec![]; n];
    for i in 0..q {
        queries_by_s[st[i].0].push(i);
    }

    let mut total_in = vec![0; q];
    let mut ft = FenwickTree::new(n);

    for s in (0..n).rev() {
        for &r in &clothes_by_l[s] {
            ft.add(r, 1);
        }
        for &qid in &queries_by_s[s] {
            let t = st[qid].1;
            total_in[qid] = ft.query(t);
        }
    }

    for i in 0..q {
        let (s, t) = st[i];

        let count_st = {
            let v = &clothes_by_l[s];
            let lower = v.partition_point(|&x| x < t);
            let upper = v.partition_point(|&x| x <= t);
            upper - lower
        };

        let mut possible = false;

        if count_st >= 1 {
            if total_in[i] >= 2 {
                possible = true;
            }
        } else {
            let r_max = {
                let v = &clothes_by_l[s];
                let idx = v.partition_point(|&x| x < t);
                if idx > 0 {
                    Some(v[idx - 1])
                } else {
                    None
                }
            };
            let l_min = {
                let v = &clothes_by_r[t];
                let idx = v.partition_point(|&x| x <= s);
                if idx < v.len() {
                    Some(v[idx])
                } else {
                    None
                }
            };

            if let (Some(rm), Some(lm)) = (r_max, l_min) {
                if rm + 1 >= lm {
                    possible = true;
                }
            }
        }

        println!("{}", if possible { "Yes" } else { "No" });
    }
}
