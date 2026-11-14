use proconio::input;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Same(i64),
    Conflict,
}

impl State {
    fn from_val(v: i64) -> Self {
        if v == -1 {
            State::Empty
        } else {
            State::Same(v)
        }
    }

    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (State::Empty, x) | (x, State::Empty) => x,
            (State::Conflict, _) | (_, State::Conflict) => State::Conflict,
            (State::Same(a), State::Same(b)) => {
                if a == b {
                    State::Same(a)
                } else {
                    State::Conflict
                }
            }
        }
    }
}

struct SegmentTree {
    size: usize,
    tree: Vec<State>,
}

impl SegmentTree {
    fn new(array: &[i64]) -> Self {
        let n = array.len();
        let size = n.next_power_of_two();
        let mut tree = vec![State::Empty; size * 2];
        for i in 0..n {
            tree[i + size] = State::from_val(array[i]);
        }
        for i in (1..size).rev() {
            tree[i] = tree[2 * i].merge(tree[2 * i + 1]);
        }
        Self { size, tree }
    }

    fn query(&self, mut l: usize, mut r: usize) -> bool {
        l += self.size;
        r += self.size;
        let mut res = State::Empty;
        while l < r {
            if l & 1 == 1 {
                res = res.merge(self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res = res.merge(self.tree[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        res != State::Conflict
    }

    fn update(&mut self, mut idx: usize, val: i64) {
        idx += self.size;
        self.tree[idx] = State::from_val(val);
        idx >>= 1;
        while idx > 0 {
            self.tree[idx] = self.tree[idx * 2].merge(self.tree[idx * 2 + 1]);
            idx >>= 1;
        }
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
    for (t, a1, a2) in queries {
        if t == 1 {
            segtree.update(a1 - 1, a2);
        } else {
            if segtree.query(a1 - 1, a2 as usize) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
