use proconio::{input, marker::Chars};
use std::collections::HashMap;

const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(0, 1), (0, N1), (1, 0), (N1, 0)];

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}
    let mut uf = UnionFind::new(h * w);
    let to_node = |y: usize, x: usize| -> usize { y * w + x };
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let node = to_node(i, j);
                for (dy, dx) in D4 {
                    let ny = i.wrapping_add(dy);
                    let nx = j.wrapping_add(dx);
                    if ny < h && nx < w && s[ny][nx] == '.' {
                        let next_node = to_node(ny, nx);
                        uf.union(node, next_node);
                    }
                }
            }
        }
    }

    let mut is_ok_root: HashMap<usize, bool> = HashMap::new();

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let node = to_node(i, j);
                let root_node: usize = uf.root(node);
                for (dy, dx) in D4 {
                    let ny = i.wrapping_add(dy);
                    let nx = j.wrapping_add(dx);
                    if ny < h && nx < w {
                        // do nothing
                    } else {
                        is_ok_root.insert(root_node, false);
                    }
                }
            }
        }
    }

    let mut ans: usize = 0;
    for (&root, is_ok) in is_ok_root.iter() {
        if *is_ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            rank: vec![0; n],
        }
    }

    // 根を返却
    pub fn root(&mut self, x: usize) -> usize {
        // parentが自分自身の場合は根
        if self.parent[x] == x {
            return x;
        }
        // 経路圧縮
        self.parent[x] = self.root(self.parent[x]);
        self.parent[x]
    }

    // xとyが同じ根か判定
    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // xとyを合体
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut rx = self.root(x);
        let mut ry = self.root(y);
        // 既に同じ
        if rx == ry {
            return false;
        }

        // ryのrankが小さくなるように調整
        // ここを省略するとrxが親になる
        if self.rank[rx] < self.rank[ry] {
            std::mem::swap(&mut rx, &mut ry);
        }
        // ryの根をrxにする
        self.parent[ry] = rx;
        // rxのrank調整
        if self.rank[rx] == self.rank[ry] {
            self.rank[rx] += 1;
        }
        self.size[rx] += self.size[ry];
        true
    }

    // xのグループの要素数
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.size[root]
    }

    // 連結かどうかを返却する
    pub fn is_linked(&mut self) -> bool {
        self.size(0) == self.size.len()
    }

    // グループの数を計算
    pub fn groups(&mut self) -> usize {
        let mut res: usize = 0;
        for x in 0..self.size.len() {
            // グループの数 = 根の数
            if x == self.root(x) {
                res += 1;
            }
        }
        res
    }
}
