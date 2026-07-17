use proconio::input;
struct Node {
    id: usize
    head: Option<Node>,
    tail: Option<Node>,
}
impl Node {
    fn new(id: usize) -> Self {
        Self {
            id: id,
            head: None,
            tail: None,
        }
    }
    fn connect_to(&mut self, other: &mut Node) {
        if let Some(source_node) = self.head {
            source_node.tail = None;
        }
        self.head = Some(*other);
        other.tail = self;
    }

    fn count_len(&self) -> usize {
        let mut count: usize = 0;
        let mut current: &Node = self;
        loop {
            let next_node: Option<&Node> = current.tail;
            match next_node {
                Some(next) => {
                    count += 1;
                    current = next;
                }
                None => break
            }
        }
        return count;
    }
}
fn main() {
    input! {n: usize, q: usize, cp: [(usize, usize); q]}
    let mut piles: Vec<Node> = 
}
