use proconio::{input, marker::Usize1};
// when implementing a union find using the roots as canonical nodes is the key.
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    const NIL: usize = usize::MAX;
    fn new(n: usize) -> Self {
        let parent: Vec<usize> = (0..n).collect();
        let size: Vec<usize> = vec![1; n];
        Self { parent, size }
    }
    fn root(&mut self, v: usize) -> usize {
        /* Conducts the path compression and returns the root of a vertext
        This uses a recursive method.
        */
        if self.parent[v] == v {
            return v;
        }
        self.parent[v] = self.root(self.parent[v]);
        return self.parent[v];
    }
    fn union(&mut self, v1: usize, v2: usize) -> bool {
        let root_v1: usize = self.root(v1);
        let root_v2: usize = self.root(v2);
        if root_v1 == root_v2 {
            return false;
        }
        if self.size[root_v1] < self.size[root_v2] {
            self.parent[root_v1] = root_v2;
            self.size[root_v2] += self.size[root_v1];
        } else {
            self.parent[root_v2] = root_v1;
            self.size[root_v1] += self.size[root_v2];
        }
        return true;
    }
    fn get_size(&mut self, v: usize) -> usize {
        let root_v: usize = self.root(v);
        return self.size[root_v];
    }
}
// abc455d
// Q. Move card "c" to the top of card "p"
// A. Reverse queries & Disjoint Set Union
// DAG of card_c -> card_p
fn main() {
    input! {n: usize, q: usize, cp: [(Usize1, Usize1); q]}
    let mut uf = UnionFind::new(n);
    let mut seen: Vec<bool> = vec![false; n];

    for &(card_c, card_p) in cp.iter().rev() {
        if !seen[card_c] {
            uf.union(card_c, card_p);
            seen[card_c] = true;
        }
    }

    for card in 0..n {
        if seen[card] {
            print!("0 ");
        } else {
            print!("{} ", uf.get_size(card));
        }
    }
    println!("");
}
