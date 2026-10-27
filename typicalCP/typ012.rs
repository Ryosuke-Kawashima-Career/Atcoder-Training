use proconio::input;
struct Unionfind {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl Unionfind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, i: usize) -> usize {
        // find the root whilst compressing the path
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        return self.parent[i];
    }
    fn unite(&mut self, u: usize, v: usize) -> bool {
        // returns true if they were in different sets before
        let root_u: usize = self.find(u);
        let root_v: usize = self.find(v);
        if root_u == root_v {
            return false;
        }
        if self.size[root_u] < self.size[root_v] {
            self.parent[root_u] = root_v;
            self.size[root_v] += self.size[root_u];
        } else {
            self.parent[root_v] = root_u;
            self.size[root_u] += self.size[root_v];
        }
        return true;
    }
    fn is_same(&mut self, u: usize, v: usize) -> bool {
        return self.find(u) == self.find(v);
    }
}
fn main() {
    input! {h: usize, w: usize, q: usize}
    let mut uf = Unionfind::new(h * w);
    let cell_to_node = |i: usize, j: usize| -> usize { w * i + j };
    let mut is_red: Vec<bool> = vec![false; h * w];
    for _ in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {mut r: usize, mut c: usize}
            r -= 1;
            c -= 1;
            is_red[cell_to_node(r, c)] = true;
            // check neighbors
            let neighbors: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (dr, dc) in neighbors {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
                    if is_red[cell_to_node(nr as usize, nc as usize)] {
                        uf.unite(cell_to_node(r, c), cell_to_node(nr as usize, nc as usize));
                    }
                }
            }
        } else {
            input! {mut ra: usize, mut ca: usize, mut rb: usize, mut cb: usize}
            ra -= 1;
            ca -= 1;
            rb -= 1;
            cb -= 1;
            if is_red[cell_to_node(ra, ca)] && is_red[cell_to_node(rb, cb)] {
                if uf.is_same(cell_to_node(ra, ca), cell_to_node(rb, cb)) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            } else {
                println!("No");
            }
        }
    }
}
