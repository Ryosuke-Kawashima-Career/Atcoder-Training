use proconio::{input, marker::Usize1};

fn main() {
    input!{n: usize, m: usize, ab: [(Usize1, Usize1); m]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in &ab {
        // directed graph
        graph[a].push(b);
    }
    let mut sc = SCC::new(&graph);
    let groups: Vec<Vec<usize>> = sc.scc();
    let mut ans: usize = 0;
    for group in groups {
        let n_members: usize = group.len();
        ans += n_members * (n_members - 1) / 2;
    }
    println!("{}", ans);
}

struct SCC {
    // Kosaraju's algorithm
    graph: Vec<Vec<usize>>,
    rev_graph: Vec<Vec<usize>>,
}

impl SCC {
    fn new(graph: &Vec<Vec<usize>>) -> Self {
        let n: usize = graph.len();
        let mut rev_graph: Vec<Vec<usize>> = vec![vec![]; n];
        for (i, edges) in graph.iter().enumerate() {
            for &j in edges {
                rev_graph[j].push(i);
            }
        }
        Self {
            graph: graph.clone(),
            rev_graph,
        }
    }

    fn scc(&mut self) -> Vec<Vec<usize>> {
        let n: usize = self.graph.len();
        let mut visited: Vec<bool> = vec![false; n];
        let mut order: Vec<usize> = Vec::new();
        for v in 0..n {
            if !visited[v] {
                self.dfs(v, &mut visited, &mut order);
            }
        }

        let mut groups: Vec<Vec<usize>> = Vec::new();
        // SCC has mutually reachable nodes, so we can use the reverse graph to find them
        let mut rev_visited: Vec<bool> = vec![false; n];
        // Process nodes in reverse order of finishing times
        for &v in order.iter().rev() {
            if !rev_visited[v] {
                let mut group: Vec<usize> = Vec::new();
                self.rdfs(v, &mut rev_visited, &mut group);
                groups.push(group);
            }
        }
        return groups;
    }
    fn dfs(&self, v: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[v] = true;
        for &u in self.graph[v].iter() {
            if !visited[u] {
                self.dfs(u, visited, order);
            }
        }
        // record the order of finishing times
        order.push(v);
    }
    fn rdfs(&self, v: usize, visited: &mut Vec<bool>, group: &mut Vec<usize>) {
        // Kosaraju's algorithm: reverse DFS to find strongly connected components
        visited[v] = true;
        group.push(v);
        for &u in self.rev_graph[v].iter() {
            if !visited[u] {
                self.rdfs(u, visited, group);
            }
        }
    }
}
