use proconio::{input, marker::Usize1};
// ABC478E
// Q. if the query type is 0: Au <= Av
// Q. if the query type is 1: Au < Av
// A. Topological Sort
struct Kosaraju {
    graph: Vec<Vec<usize>>,
    graph_rev: Vec<Vec<usize>>,
}
impl Kosaraju {
    fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n: usize = graph.len();
        let mut graph_rev: Vec<Vec<usize>> = vec![vec![]; n];
        for from in 0..n {
            for &to in graph[from].iter() {
                graph_rev[to].push(from);
            }
        }
        Self {
            graph: graph.clone(),
            graph_rev,
        }
    }
    fn topological_sort(&self) -> Vec<usize> {
        let n: usize = self.graph.len();
        let mut seen: Vec<bool> = vec![false; n];
        let mut order: Vec<usize> = Vec::new();
        for v in 0..n {
            if !seen[v] {
                self.dfs_forward(v, &mut seen, &mut order);
            }
        }
        // here order is in reverse topological order
        order.reverse();
        // here order is in topological order
        order
    }
    fn get_scc(&self) -> Vec<Vec<usize>> {
        let topological_order: Vec<usize> = self.topological_sort();
        let n: usize = self.graph.len();
        let mut components: Vec<Vec<usize>> = Vec::new();
        let mut seen_rev: Vec<bool> = vec![false; n];
        for &v in topological_order.iter() {
            if !seen_rev[v] {
                let mut component: Vec<usize> = Vec::new();
                self.dfs_backward(v, &mut seen_rev, &mut component);
                components.push(component);
            }
        }
        components
    }
    fn dfs_forward(&self, v: usize, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
        seen[v] = true;
        for &next in self.graph[v].iter() {
            if !seen[next] {
                self.dfs_forward(next, seen, order);
            }
        }
        // post-order
        order.push(v);
    }
    fn dfs_backward(&self, v: usize, seen_rev: &mut Vec<bool>, component: &mut Vec<usize>) {
        seen_rev[v] = true;
        for &next in self.graph_rev[v].iter() {
            if !seen_rev[next] {
                self.dfs_backward(next, seen_rev, component);
            }
        }
        component.push(v);
    }
}
fn main() {
    input! {n: usize, q: usize, queries: [(usize, Usize1, Usize1); q]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut lt_edges: Vec<(usize, usize)> = Vec::new();
    for &(query_type, u, v) in queries.iter() {
        graph[u].push(v);
        if query_type == 1 {
            lt_edges.push((u, v));
        }
    }
    let kosaraju = Kosaraju::new(&graph);
    let scc: Vec<Vec<usize>> = kosaraju.get_scc();
    let mut labels: Vec<usize> = vec![0; n];
    for (i, components) in scc.iter().enumerate() {
        for &node in components.iter() {
            labels[node] = i + 1;
        }
    }
    // check if there is any contradiction regarding the less than edges
    for &(u, v) in lt_edges.iter() {
        if labels[u] == labels[v] {
            println!("No");
            return;
        }
    }
    println!("Yes");
    for v in 0..n {
        print!("{} ", labels[v]);
    }
    println!("");
}
