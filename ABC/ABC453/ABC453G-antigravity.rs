use proconio::input;
/* ABC453G: Persistent Segment Tree
You are given a 2D matrix (N x M).
Query1: Change `A[x]` to `A[y]`.
Query2: Change `A[x][y]` to `z`.
Query3: Get sum of `A[x][l..r]`. */
// Custom Node for a highly optimized Persistent Segment Tree
#[derive(Clone, Copy)]
struct Node {
    ls: u32,
    rs: u32,
    sum: i64,
}

fn update(tree: &mut Vec<Node>, node: u32, l: usize, r: usize, idx: usize, val: i64) -> u32 {
    let (ls, rs, _) = if node != 0 {
        let n = tree[node as usize];
        (n.ls, n.rs, n.sum)
    } else {
        (0, 0, 0)
    };

    if l == r {
        let new_node_idx = tree.len() as u32;
        tree.push(Node {
            ls: 0,
            rs: 0,
            sum: val,
        });
        return new_node_idx;
    }

    let mut new_ls = ls;
    let mut new_rs = rs;
    let mid = l + (r - l) / 2;

    if idx <= mid {
        new_ls = update(tree, ls, l, mid, idx, val);
    } else {
        new_rs = update(tree, rs, mid + 1, r, idx, val);
    }

    let sum = tree[new_ls as usize].sum + tree[new_rs as usize].sum;
    let new_node_idx = tree.len() as u32;
    tree.push(Node {
        ls: new_ls,
        rs: new_rs,
        sum,
    });

    new_node_idx
}

fn query(tree: &[Node], node: u32, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
    if node == 0 {
        return 0;
    }
    if ql <= l && r <= qr {
        return tree[node as usize].sum;
    }

    let mut sum = 0;
    let mid = l + (r - l) / 2;

    if ql <= mid {
        sum += query(tree, tree[node as usize].ls, l, mid, ql, qr);
    }
    if qr > mid {
        sum += query(tree, tree[node as usize].rs, mid + 1, r, ql, qr);
    }
    sum
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    }

    // Initialize tracking arrays
    let mut tree = Vec::with_capacity(q * 20 + 2);
    tree.push(Node {
        ls: 0,
        rs: 0,
        sum: 0,
    }); // Null Node structurally equal 0

    let mut roots = vec![0u32; n + 1];

    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! { x: usize, y: usize }
        } else if t == 2 {
            input! { x: usize, y: usize, z: i64 }
            roots[x] = update(&mut tree, roots[x], 1, m, y, z);
        } else {
            input! { x: usize, l: usize, r: usize }
            println!("{}", query(&tree, roots[x], 1, m, l, r));
        }
    }
}
