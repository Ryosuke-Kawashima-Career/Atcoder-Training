use proconio::{input, marker::Usize1};
use std::collections::{BTreeSet, HashMap};

fn main() {
    input! {n: usize, q: usize, mut a: [i64; n], queries: [(Usize1, i64); q]}
    let mut mex_set = MexSet::new();
    for i in 0..n {
        mex_set.add(a[i]);
    }
    let mut map: HashMap<i64, usize> = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    for &(index, element) in queries.iter() {
        if let Some(c) = map.get_mut(&a[index]) {
            *c -= 1;
            if *c == 0 {
                mex_set.remove(a[index]);
            }
        }
        a[index] = element;
        let c = map.entry(a[index]).or_insert(0);
        *c += 1;
        if *c == 1 {
            mex_set.add(a[index]);
        }
        println!("{}", mex_set.get_mex());
    }
}

struct MexSet {
    // stores disjoint intervals [l, r) covering integers.
    set: BTreeSet<(i64, i64)>,
}

impl MexSet {
    const INF: i64 = 1i64 << 60;

    pub fn new() -> Self {
        Self {
            set: BTreeSet::new(),
        }
    }

    pub fn contains(&self, element: i64) -> bool {
        /* Check if the element is contained in the set
        Args:
            element: the element to check
        Returns:
            true if the element is contained in the set
            false if the element is not contained in the set
        */
        match self.set.range(..(element + 1, Self::INF)).next_back() {
            Some(&(l, r)) => l <= element && element < r,
            None => false,
        }
    }

    pub fn range_covered(&self, l: i64, r: i64) -> bool {
        /* Check if the range [l, r) is covered by the set
        Args:
            l: the left bound of the range
            r: the right bound of the range
        Returns:
            true if the range is covered by the set
            false if the range is not covered by the set
        */
        if l >= r {
            return false;
        }
        match self.set.range(..(l + 1, Self::INF)).next_back() {
            Some(&(range_left, range_right)) => range_left <= l && r <= range_right,
            None => false,
        }
    }

    pub fn add(&mut self, element: i64) -> bool {
        /*Adds [element, element+1) to the set
        Args:
            element: the element to be added
        Returns:
            true if the element was added successfully
            false if the element was already in the set
        */
        if self.contains(element) {
            return false;
        }
        let mut new_l = element;
        let mut new_r = element + 1;

        // Try to merge with the left interval: [l, r) where r == element
        if let Some(&(l, r)) = self.set.range(..(element, Self::INF)).next_back() {
            if r == element {
                new_l = l;
                self.set.remove(&(l, r));
            }
        }

        // Try to merge with the right interval: [l, r) where l == element + 1
        if let Some(&(l, r)) = self.set.range((element + 1, -Self::INF)..).next() {
            if l == element + 1 {
                new_r = r;
                self.set.remove(&(l, r));
            }
        }

        self.set.insert((new_l, new_r));
        true
    }

    pub fn remove(&mut self, element: i64) -> bool {
        /*Removes [element, element+1) from the set
        Args:
            element: the element to be removed
        Returns:
            true if the element was removed successfully
            false if the element was not in the set
        */
        let target_interval = match self.set.range(..(element + 1, Self::INF)).next_back() {
            Some(&(l, r)) if l <= element && element < r => Some((l, r)),
            _ => None,
        };

        if let Some((l, r)) = target_interval {
            self.set.remove(&(l, r));
            if l < element {
                self.set.insert((l, element));
            }
            if element + 1 < r {
                self.set.insert((element + 1, r));
            }
            true
        } else {
            false
        }
    }

    pub fn get_mex(&self) -> i64 {
        /*Calculates the minimum non-negative excludant
        Returns:
            the minimum excludant
         */
        match self.set.range(..(1, Self::INF)).next_back() {
            Some(&(l, r)) => {
                if l <= 0 && 0 < r {
                    r
                } else {
                    0
                }
            }
            None => 0,
        }
    }
}
