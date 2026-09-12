use proconio::input;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
// Pointer itself can be None.
type Link = Option<Rc<RefCell<Node>>>;
type WeakLink = Option<Weak<RefCell<Node>>>;

pub struct Node {
    pub val: usize,
    pub prev: WeakLink,
    pub next: Link,
}
impl Node {
    // Recursive definition
    fn new(val: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            prev: None,
            next: None,
        }))
    }
}

pub struct BiLinkedList {
    pub head: Link,
    pub tail: Link,
    pub map: HashMap<usize, Rc<RefCell<Node>>>,
}
impl BiLinkedList {
    pub fn new(p: &[usize]) -> Self {
        let mut map: HashMap<usize, Rc<RefCell<Node>>> = HashMap::new();
        if p.is_empty() {
            return Self {
                head: None,
                tail: None,
                map,
            };
        }
        let head: Link = Some(Node::new(p[0]));
        // Clone the reference
        map.insert(p[0], Rc::clone(&head));
        let mut curr: Link = Rc::clone(&head);
        for &val in &p[1..] {
            let next_node = Node::new(val);
            // Link <-> next_node
            curr.borrow_mut().next = Some(Rc::clone(&next_node));
            next_node.borrow_mut().prev = Some(Rc::downgrade(&curr));
            // note that next_node is a reference
            curr = Some(next_node.clone());
            map.insert(val, Rc::clone(next_node));
        }
        let tail: Link = Some(Rc::clone(curr));
        Self { head, tail, map }
    }
    fn to_tail(&mut self, target: usize) {
        let target_node: Link = match self.map.get(&target) {
            Some(node) => Some(Rc::clone(node)),
            None => {
                return;
            }
        };
        // if the target is already tail, return.
        if let Some(ref tail_node) = self.tail {
            if Rc::ptr_eq(&target_node, tail_node) {
                return;
            }
        }
        let prev_opt: Link = target_node
            .borrow_mut()
            .prev
            .as_ref()
            .and_then(|weak_ref| weak_ref.upgrade());
        let next_opt: Link = target_node.borrow_mut().next.clone();
        match prev_opt {
            Some(prev_node) => {
                prev_node.borrow_mut().next = next_opt.clone();
            }
            None => {
                self.head = next_opt.clone();
            }
        }
        match next_opt {
            Some(next_node) => {
                next_node.borrow_mut().prev = Some(Rc::downgrade(&prev_opt));
            }
            None => {
                self.tail = prev_opt.clone();
            }
        }
    }
    fn display(&self) {
        let mut curr: Link = self.head.clone();
        while let Some(node) = curr {
            print!("{} ", node.borrow().val);
            curr = node.borrow().next.clone();
        }
        println!();
    }
}
fn main() {
    input! {n: usize, q: usize, p: [usize; n], aq: [usize; q]}
    let mut list = BiLinkedList::new(&p);
    for query in 0..q {
        let target: usize = aq[query];
        list.to_tail(target);
    }
    list.display();
}
