use proconio::input;
struct Kosaraju {
    graph: Vec<Vec<usize>>,
    rev_graph: Vec<Vec<usize>>,
}
impl Kosaraju {
    fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n: usize = graph.len();
        let graph: Vec<Vec<usize>> = graph.clone();
        let mut rev_graph: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 0..n {
            for &to in graph[i].iter() {
                rev_graph[to].push(i);
            }
        }
        Self { graph, rev_graph }
    }
    fn get_topological(&self) -> Vec<usize> {
        let n: usize = self.graph.len();
        let mut visited: Vec<bool> = vec![false; n];
        let mut order: Vec<usize> = Vec::new();
        for v in 0..n {
            if !visited[v] {
                self.dfs(v, &mut visited, &mut order);
            }
        }
        order.reverse();
        order
    }
    fn get_scc(&self) -> Vec<Vec<usize>> {
        let order: Vec<usize> = self.get_topological();
        let n: usize = self.graph.len();
        let mut visited: Vec<bool> = vec![false; n];
        let mut scc: Vec<Vec<usize>> = Vec::new();
        for v in order {
            if !visited[v] {
                let mut scc_v: Vec<usize> = Vec::new();
                self.dfs_rev(v, &mut visited, &mut scc_v);
                scc.push(scc_v);
            }
        }
        scc
    }
    fn dfs(&self, v: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[v] = true;
        for &to in self.graph[v].iter() {
            if !visited[to] {
                self.dfs(to, visited, order);
            }
        }
        order.push(v);
    }
    fn dfs_rev(&self, v: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[v] = true;
        for &to in self.rev_graph[v].iter() {
            if !visited[to] {
                self.dfs_rev(to, visited, order);
            }
        }
        order.push(v);
    }
}
fn main() {
    input! {n: usize, q: usize, tuv: [(usize, usize, usize); q]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut weighted_edges: Vec<(usize, usize)> = Vec::new();
    for &(query_type, u, v) in tuv.iter() {
        if query_type == 0 {
            graph[u - 1].push(v - 1);
        } else {
            graph[u - 1].push(v - 1);
            weighted_edges.push((u - 1, v - 1));
        }
    }
    let kosaraju = Kosaraju::new(&graph);
    let scc: Vec<Vec<usize>> = kosaraju.get_scc();

    let mut scc_id: Vec<usize> = vec![0; n];
    for (i, scc_v) in scc.iter().enumerate() {
        for &v in scc_v.iter() {
            scc_id[v] = i;
        }
    }
    for &(from, to) in weighted_edges.iter() {
        if scc_id[from] == scc_id[to] {
            println!("No");
            return;
        }
    }
    println!("Yes");
    for v in 0..n {
        print!("{} ", scc_id[v] + 1);
    }
    println!("");
}
