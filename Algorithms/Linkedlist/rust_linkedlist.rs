use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Link = Option<Rc<RefCell<Node>>>;
pub type WeakLink = Option<Weak<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub val: i64,
    pub next: Link,
    pub prev: WeakLink,
}

impl Node {
    pub fn new(id: usize, val: i64) -> Rc<RefCell<Self>> {
        // Creates a new node wrapped in `Rc<RefCell<Self>>`.
        Rc::new(RefCell::new(Self {
            id,
            val,
            next: None,
            prev: None,
        }))
    }

    pub fn connect_to(this: &Rc<RefCell<Self>>, other: &Rc<RefCell<Self>>) {
        /* Connects `this` node to `other` node (`this -> other`).
        Safely clears existing connections to prevent dangling or invalid pointers. */
        // 1. Disconnect old `next` node of `this`
        if let Some(old_next) = this.borrow_mut().next.take() {
            old_next.borrow_mut().prev = None;
        }

        // 2. Disconnect old `prev` node of `other`
        if let Some(old_prev_weak) = other.borrow_mut().prev.take() {
            if let Some(old_prev) = old_prev_weak.upgrade() {
                old_prev.borrow_mut().next = None;
            }
        }

        // 3. Establish double link between `this` and `other`
        this.borrow_mut().next = Some(Rc::clone(other));
        other.borrow_mut().prev = Some(Rc::downgrade(this));
    }

    pub fn count_len(this: &Rc<RefCell<Self>>) -> usize {
        // Calculates the number of nodes reachable by traversing forward from `this`.
        let mut length = 0;
        let mut curr = Some(Rc::clone(this));

        while let Some(node_rc) = curr {
            length += 1;
            let next_node = node_rc.borrow().next.clone();
            curr = next_node;
        }

        length
    }
}

fn main() {
    let node1 = Node::new(1, 100);
    let node2 = Node::new(2, 200);
    let node3 = Node::new(3, 300);

    Node::connect_to(&node1, &node2);
    Node::connect_to(&node2, &node3);

    println!("Length from node1: {}", Node::count_len(&node1)); // Output: 3
    println!("Length from node2: {}", Node::count_len(&node2)); // Output: 2
}
