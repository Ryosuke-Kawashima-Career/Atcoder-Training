use proconio::{input, marker::Usize1};
// pre-read all the queries before hand
fn main() {
    input! {n: usize, q: usize, queries: [(usize, Usize1, Usize1, i64)]}
    let mut uf = Unionfind::new(n);
    let mut sums: Vec<i64> = vec![0; n - 1];
    let mut crossing_sums: Vec<i64> = vec![0; n];

    for query in queries.iter() {
        let (query_type, index_x, index_y, value) = (query.0, query.1, query.2, query.3);
        if query_type == 0 {
            uf.unite(index_x, index_y);
            sums[index_x] = value;
        }
    }
    for i in 0..n - 1 {
        crossing_sums[i + 1] = sums[i] - crossing_sums[i];
    }

    for query in queries.iter() {
        let (query_type, index_x, index_y, value) = (query.0, query.1, query.2, query.3);
        if query_type == 1 {
            if uf.find(index_x) == uf.find(index_y) {
                let diff_xy: isize = (index_x as isize - index_y as isize).abs();
                if diff_xy % 2 == 0 {
                    let answer: i64 = value + crossing_sums[index_y] - crossing_sums[index_x];
                    println!("{}", answer);
                } else {
                    let answer: i64 = crossing_sums[index_x] - crossing_sums[index_y];
                    println!("{}", answer);
                }
            } else {
                println!("Ambiguous");
            }
        }
    }
}

struct Unionfind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Unionfind {
    fn new(n: usize) -> Self {
        let parent: Vec<usize> = (0..n).collect();
        let size: Vec<usize> = vec![1; n];
        Self { parent, size }
    }

    fn find(&mut self, index: usize) -> usize {
        if self.parent[index] == index {
            return index;
        }
        let parent = self.parent[index];
        let root = self.find(parent);
        self.parent[index] = root;
        root
    }

    fn unite(&mut self, left: usize, right: usize) -> bool {
        // return true if this action merged two different components
        let left_root = self.find(left);
        let right_root = self.find(right);

        if left_root == right_root {
            return false;
        }

        if self.size[left_root] < self.size[right_root] {
            self.parent[left_root] = self.parent[right_root];
            self.size[right_root] += self.size[left_root];
        } else {
            self.parent[right_root] = self.parent[left_root];
            self.size[left_root] += self.size[right_root];
        }

        return true;
    }
}
