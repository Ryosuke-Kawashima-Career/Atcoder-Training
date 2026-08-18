struct Heap<T: Ord> {
    /* Return the minimum value if any */
    data: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn heapify(arr: &[T]) -> Self
    where
        T: Clone,
    {
        let mut heap = Self {
            data: arr.to_vec(),
        };
        if heap.data.len() > 1 {
            for i in (0..heap.data.len() / 2).rev() {
                heap.sift_down(i);
            }
        }
        heap
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.sift_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        // Swap root with the last element for O(1) removal, then sift down the new root
        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);
        let top = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        top
    }

    pub fn sift_up(&mut self, mut idx: usize) {
        /* This method is called when a new value is added. */
        while idx > 0 {
            let parent_idx: usize = (idx - 1) / 2;
            if self.data[parent_idx] > self.data[idx] {
                self.data.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    pub fn sift_down(&mut self, mut idx: usize) {
        /* This method is called to restore the min-heap property downwards */
        let n = self.data.len();
        loop {
            // Going for the children of the node (idx)
            let left_idx = 2 * idx + 1;
            let right_idx = 2 * idx + 2;
            let mut min_idx = idx;

            if left_idx < n && self.data[left_idx] < self.data[min_idx] {
                min_idx = left_idx;
            }
            if right_idx < n && self.data[right_idx] < self.data[min_idx] {
                min_idx = right_idx;
            }

            if min_idx != idx {
                self.data.swap(min_idx, idx);
                idx = min_idx;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop_sorted() {
        let mut heap = Heap::new();
        let values = vec![5, 3, 8, 1, 2, 9, 4, 7, 6];
        for &v in &values {
            heap.push(v);
        }

        let mut popped = Vec::new();
        while let Some(v) = heap.pop() {
            popped.push(v);
        }

        assert_eq!(popped, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_heapify() {
        let values = vec![9, 4, 7, 1, -2, 6, 3, 5, 0];
        let mut heap = Heap::heapify(&values);

        let mut popped = Vec::new();
        while let Some(v) = heap.pop() {
            popped.push(v);
        }

        let mut expected = values.clone();
        expected.sort();
        assert_eq!(popped, expected);
    }

    #[test]
    fn test_peek_and_empty() {
        let mut heap: Heap<i32> = Heap::new();
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
        assert!(heap.is_empty());

        heap.push(10);
        assert_eq!(heap.peek(), Some(&10));
        assert_eq!(heap.len(), 1);

        heap.push(5);
        assert_eq!(heap.peek(), Some(&5));

        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.peek(), Some(&10));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_duplicates() {
        let mut heap = Heap::new();
        let values = vec![4, 2, 4, 1, 2, 1];
        for &v in &values {
            heap.push(v);
        }

        let mut popped = Vec::new();
        while let Some(v) = heap.pop() {
            popped.push(v);
        }

        assert_eq!(popped, vec![1, 1, 2, 2, 4, 4]);
    }
}