use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, q: usize, cp: [(Usize1, Usize1); q]}
    let mut list: Vec<LinkedList<usize>> = vec![LinkedList::new(); n];
    for i in 0..n {
        list[i].push_back(i);
    }

    for _ in 0..q {
        input! {c: Usize1, p: Usize1}
        list[c].insert_next(p)
    }
}

#[derive(Debug, Clone, Copy)]
struct Link {
    id: usize,
    prev: usize,
    next: usize,
}

#[derive(Debug)]
struct Node<T> {
    id: usize,
    prev: usize,
    next: usize,
    value: T,
}

impl<T> Node<T> {
    pub fn link(&self) -> Link {
        Link {
            id: self.id,
            prev: self.prev,
            next: self.next,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    list: Vec<Node<T>>,
}

impl<T: Default> LinkedList<T> {
    pub fn new() -> Self {
        let mut list = vec![];
        list.push(Node {
            id: Self::HEAD,
            prev: Self::NIL,
            next: Self::TAIL,
            value: T::default(),
        });
        list.push(Node {
            id: Self::TAIL,
            prev: Self::HEAD,
            next: Self::NIL,
            value: T::default(),
        });
        Self { list }
    }
}

impl<T> LinkedList<T> {
    const NIL: usize = usize::MAX;
    const HEAD: usize = 0;
    const TAIL: usize = 1;

    pub fn push_back(&mut self, value: T) -> usize {
        self.insert_next(self.list[Self::TAIL].prev, value)
    }
    pub fn push_front(&mut self, value: T) -> usize {
        self.insert_prev(self.list[Self::HEAD].next, value)
    }

    pub fn remove(&mut self, id: usize) {
        assert!(2 <= id && id < self.list.len());
        let link = self.list[id].link();
        self.list[link.prev].next = link.next;
        self.list[link.next].prev = link.prev;
    }

    pub fn insert_next(&mut self, id: usize, value: T) -> usize {
        assert!(id != Self::TAIL && id < self.list.len());
        // a -> b -> c
        let a = self.list[id].link();
        let c = self.list[a.next].link();

        let id_of_b = self.list.len();
        let b = Node {
            id: id_of_b,
            prev: a.id,
            next: c.id,
            value,
        };

        self.list[a.id].next = id_of_b;
        self.list[c.id].prev = id_of_b;
        self.list.push(b);
        id_of_b
    }
    pub fn insert_prev(&mut self, id: usize, value: T) -> usize {
        assert!(id != Self::HEAD && id < self.list.len());
        // a -> b -> c
        let c = self.list[id].link();
        let a = self.list[c.prev].link();

        let id_of_b = self.list.len();
        let b = Node {
            id: id_of_b,
            prev: a.id,
            next: c.id,
            value,
        };

        self.list[a.id].next = id_of_b;
        self.list[c.id].prev = id_of_b;
        self.list.push(b);
        id_of_b
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        use std::marker::PhantomData;
        struct Iter<'a, T> {
            id: usize,
            linked_list: &'a LinkedList<T>,
            phantom: PhantomData<T>,
        }

        impl<'a, T> Iterator for Iter<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                self.id = self.linked_list.list[self.id].next;
                if self.id == LinkedList::<T>::TAIL {
                    None
                } else {
                    Some(&self.linked_list.list[self.id].value)
                }
            }
        }

        Iter {
            id: Self::HEAD,
            linked_list: self,
            phantom: PhantomData,
        }
    }
}
