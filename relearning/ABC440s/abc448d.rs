use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;
fn main() {
    input! {n: usize, a: [usize; n], edges: [(Usize1, Usize1); n-1]}
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for (u, v) in edges {
        tree[u].push(v);
        tree[v].push(u);
    }
    let mut duplicated: Vec<bool> = vec![false; n];
    let mut map: HashMap<usize, usize> = HashMap::new();
    dfs(
        0,
        usize::MAX,
        false,
        &tree,
        &a,
        &mut duplicated,
        &mut map,
    );
    for v in 0..n {
        if duplicated[v] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn dfs(
    v: usize,
    parent: usize,
    has_dup: bool,
    tree: &Vec<Vec<usize>>,
    label: &Vec<usize>,
    duplicated: &mut Vec<bool>,
    map: &mut HashMap<usize, usize>,
) {
    let entry = map.entry(label[v]).or_insert(0);
    *entry += 1;
    let current_has_dup = has_dup || (*entry > 1);
    duplicated[v] = current_has_dup;

    for &next in tree[v].iter() {
        if next != parent {
            dfs(next, v, current_has_dup, tree, label, duplicated, map);
        }
    }
    *map.get_mut(&label[v]).unwrap() -= 1;
}
