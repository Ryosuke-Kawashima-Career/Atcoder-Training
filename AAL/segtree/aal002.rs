use proconio::input;
#[derive(Debug, Clone, Copy)]
struct Node {
    sum: i64,
    len: i64,
}
impl Node {
    const E: Node = Node { sum: 0, len: 0 };
    fn merge(&self, other: Node) -> Node {
        return Node {
            sum: self.sum + other.sum,
            len: self.len + other.len,
        };
    }
}
#[derive(Debug, Clone, Copy)]
struct Action {
    mul: i64,
    add: i64,
}
impl Action {
    const E: Action = Action { mul: 1, add: 0 };
    fn compose(&self, other: Action) -> Action {
        return Action {
            mul: self.mul * other.mul,
            add: self.add * other.mul + other.add,
        };
    }
    fn apply(&self, node: Node) -> Node {
        return Node {
            sum: node.sum * self.mul + self.add * node.len,
            len: node.len,
        };
    }
}
struct LazySegmentTree {
    data: Vec<Node>,
    lazy: vec<Action>,
    size: usize,
}
impl LazySegmentTree {
    fn new(a: &[i64]) -> LazySegmentTree {
        let n: usize = a.len();
        let size: usize = n.next_power_of_two();
        let mut data = vec![Node::E; 2 * size];
        let mut lazy = vec![Action::E; 2 * size];
        for i in 0..n {
            data[size + i] = Node { sum: a[i], len: 1 };
        }
        for i in (0..self.size).rev() {
            data[i] = data[2 * i].merge(data[2 * i + 1]);
        }
        return LazySegmentTree {
            data: data,
            lazy: lazy,
            size: size,
        };
    }
}
fn main() {
    input! {n: usize, q: usize, a: [i64; n]}
    let mut segtree = LazySegmentTree::new(&a);
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {l: usize, r: usize, x: i64}
            let (l, r) = (l - 1, r);
            segtree.update(l, r, x);
        } else {
            input! {l: usize, r: usize}
            let (l, r) = (l - 1, r);
            let result = segtree.query(l, r);
            println!("{}", result);
        }
    }
}
