use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, q: usize, cp: [(Usize1, Usize1); q]}
    let mut list: Vec<Option<Node>> = vec![None; n];
    for i in 0..n {
        list[i] = Some(Node::new(i, i, Node::HEAD, Node::TAIL));
    }

    for _ in 0..q {
        input! {c: Usize1, p: Usize1}
        list[p] = (list[c].unwrap()).merge(list[p]);
        list[c] = None;
    }
    for i in 0..n {
        if let Some(node) = list[i] {
            let ans = node.get_length();
            print!("{} ", ans);
        } else {
            print!("0");
        }
        println!();
    }
}

#[derive(Debug, Copy, Clone)]
struct Node {
    id: usize,
    value: usize,
    prev_id: usize,
    next_id: usize,
}

impl Node {
    const HEAD: usize = usize::MAX - 1;
    const TAIL: usize = usize::MAX;
    fn new(id: usize, value: usize, prev_id: usize, next_id: usize) -> Self {
        Self {
            id,
            value,
            prev_id,
            next_id,
        }
    }

    fn merge(self, other: Option<Self>) -> Option<Self> {
        match other {
            Some(other_node) => {
                let mut c_node = self;
                let mut p_node = other_node;
                c_node.next_id = p_node.id;
                p_node.prev_id = c_node.id;
                Some(c_node)
            }
            None => Some(self),
        }
    }

    fn get_length(&self) -> usize {
        let mut count = 0;
        let mut current = self.id;
        while current != Self::TAIL {
            count += 1;
            current = self.next_id;
        }
        count
    }
}
