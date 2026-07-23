use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Option<Rc<RefCell<Node>>>;
type WeakLink = Option<Weak<RefCell<Node>>>;

struct Node {
    id: usize,
    val: i64,
    next: Link,
    prev: WeakLink,
}

impl Node {
    fn new(id: usize, val: i64) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id,
            val,
            next: None,
            prev: None,
        }))
    }
    fn connect_to(this: &Rc<RefCell<Self>>, other: &Rc<RefCell<Self>>) {
        /* Connect this to other */
        if let Some(old_next) = &this.borrow().next {
            old_next.borrow_mut().prev = None;
        }
        if let Some(old_prev) = &other.borrow().prev {
            old_prev.borrow_mut().next = None;
        }
        this.borrow_mut().next = Some(Rc::clone(other));
        // Link -> WeakLink -> RefCell -> WeakLink
        other.borrow_mut().prev = Some(Rc::downgrade(this));
    }
    fn count_len(this: &Rc<RefCell<Self>>) -> usize {
        /* Calculate the length of the Link */
        let mut length: usize = 1;
        if this.is_empty() {
            return 0;
        }
        let mut curr: Link = Some(Rc::clone(this));
        while let Some(next) = curr.borrow().next {
            length += 1;
            curr = Some(Rc::clone(&next));
        }
        length
    }
}
