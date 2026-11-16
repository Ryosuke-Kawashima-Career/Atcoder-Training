use proconio::{input, marker::Usize1};
// Inversing the subject and object
// calculate the contribution of each entity
const NIL: usize = usize::MAX;
fn main() {
    input!{n: usize, ab: [(Usize1, Usize1); n-1]}
    let mut ans: usize = 0;
    let mut subsize: Vec<usize> = vec![0; n];
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut visited: Vec<bool> = vec![false; n];
    dfs(0, NIL, &graph, &mut subsize, &mut visited);
    for v in 0..n {
        ans += (n - subsize[v]) * subsize[v];
    }
    println!("{}", ans);
}

fn dfs(v: usize, parent: usize, graph: &Vec<Vec<usize>>, subsize: &mut Vec<usize>, visited: &mut Vec<bool>) -> usize {
    if visited[v] {
        return 0;
    }
    visited[v] = true;
    let mut members: usize = 1;
    for &next in graph[v].iter() {
        let descendants: usize = dfs(next, v, graph, subsize, visited);
        members += descendants;
    }
    subsize[v] = members;
    return subsize[v];
}
