/// Tarjan SCC decomposition with component IDs in source-to-sink order.
/// Construction takes O(V + E) time and O(V) auxiliary space.
/// For every edge u -> v between different SCCs, ids[u] < ids[v].
pub struct Tarjan {
    graph: Vec<Vec<usize>>,
    ids: Vec<usize>,
    scc_clusters: Vec<Vec<usize>>,
}

impl Tarjan {
    pub fn new(graph: &[Vec<usize>]) -> Self {
        let n = graph.len();
        let mut discovery = vec![usize::MAX; n];
        let mut low_link = vec![0; n];
        let mut on_stack = vec![false; n];
        let mut scc_stack = Vec::with_capacity(n);
        // Each frame stores (vertex, index of its next outgoing edge).
        // This stack replaces recursive DFS calls; it is not the SCC stack.
        let mut dfs_stack = Vec::with_capacity(n);
        let mut timer = 0;
        let mut scc_clusters = Vec::new();

        for start in 0..n {
            if discovery[start] != usize::MAX {
                continue;
            }
            discovery[start] = timer;
            low_link[start] = timer;
            timer += 1;
            on_stack[start] = true;
            scc_stack.push(start);
            dfs_stack.push((start, 0));

            while let Some(&(v, next_index)) = dfs_stack.last() {
                if next_index < graph[v].len() {
                    let next = graph[v][next_index];
                    dfs_stack.last_mut().unwrap().1 += 1;
                    if discovery[next] == usize::MAX {
                        discovery[next] = timer;
                        low_link[next] = timer;
                        timer += 1;
                        on_stack[next] = true;
                        scc_stack.push(next);
                        dfs_stack.push((next, 0));
                    } else if on_stack[next] {
                        // Use discovery[next] for an already visited stack vertex.
                        low_link[v] = low_link[v].min(discovery[next]);
                    }
                } else {
                    // Finish v only after every outgoing edge has been processed.
                    dfs_stack.pop();
                    if low_link[v] == discovery[v] {
                        let mut component = Vec::new();
                        loop {
                            let node = scc_stack.pop().unwrap();
                            on_stack[node] = false;
                            component.push(node);
                            if node == v {
                                break;
                            }
                        }
                        scc_clusters.push(component);
                    }
                    if let Some(&(parent, _)) = dfs_stack.last() {
                        // Equivalent to returning from a recursive tree-edge call.
                        low_link[parent] = low_link[parent].min(low_link[v]);
                    }
                }
            }
        }

        // Tarjan completes sink SCCs first. Reverse them to put sources first.
        scc_clusters.reverse();
        let mut ids = vec![0; n];
        for (id, component) in scc_clusters.iter().enumerate() {
            for &v in component {
                ids[v] = id;
            }
        }
        Self {
            graph: graph.to_vec(),
            ids,
            scc_clusters,
        }
    }

    /// Zero-based SCC ID for each vertex, in topological order.
    pub fn get_topological_ids(&self) -> &[usize] {
        &self.ids
    }

    /// SCCs in topological order. Vertex order inside an SCC is unspecified.
    pub fn clusters(&self) -> &[Vec<usize>] {
        &self.scc_clusters
    }

    /// Component IDs in source-to-sink order, not a vertex topological order.
    pub fn topological_order(&self) -> std::ops::Range<usize> {
        0..self.scc_clusters.len()
    }

    /// Build the SCC DAG, omitting self-edges and duplicate edges.
    /// Takes O(V + E) time; outgoing neighbors are not necessarily sorted.
    pub fn condensation_graph(&self) -> Vec<Vec<usize>> {
        let count = self.scc_clusters.len();
        let mut dag = vec![Vec::new(); count];
        let mut last_source = vec![usize::MAX; count];
        for (from, component) in self.scc_clusters.iter().enumerate() {
            for &v in component {
                for &next in &self.graph[v] {
                    let to = self.ids[next];
                    if from != to && last_source[to] != from {
                        last_source[to] = from;
                        dag[from].push(to);
                    }
                }
            }
        }
        dag
    }
}

fn main() {
    // {0,1} -> {2,3} -> {4}
    let graph = vec![vec![1], vec![0, 2], vec![3], vec![2, 4], vec![]];
    let tarjan = Tarjan::new(&graph);
    println!("Component IDs: {:?}", tarjan.get_topological_ids());
    println!("SCCs: {:?}", tarjan.clusters());
    println!("Condensed DAG: {:?}", tarjan.condensation_graph());
    for id in tarjan.topological_order() {
        println!("Component {}: {:?}", id, tarjan.clusters()[id]);
    }
}

#[cfg(test)]
mod tests {
    use super::Tarjan;

    #[test]
    fn exhaustive_small_graphs() {
        // Includes disconnected graphs, self-loops, cycles, and cross edges.
        for n in 0..=4 {
            for mask in 0usize..(1usize << (n * n)) {
                let mut graph = vec![Vec::new(); n];
                let mut reachable = vec![vec![false; n]; n];
                for u in 0..n {
                    reachable[u][u] = true;
                    for v in 0..n {
                        if mask & (1 << (u * n + v)) != 0 {
                            graph[u].push(v);
                            reachable[u][v] = true;
                        }
                    }
                }
                for k in 0..n {
                    for u in 0..n {
                        for v in 0..n {
                            reachable[u][v] |= reachable[u][k] && reachable[k][v];
                        }
                    }
                }
                let result = Tarjan::new(&graph);
                let ids = result.get_topological_ids();
                let dag = result.condensation_graph();
                let mut occurrences = vec![0; n];
                for (id, cluster) in result.clusters().iter().enumerate() {
                    assert!(!cluster.is_empty());
                    for &v in cluster {
                        assert_eq!(ids[v], id);
                        occurrences[v] += 1;
                    }
                }
                assert!(occurrences.iter().all(|&count| count == 1));
                for u in 0..n {
                    for v in 0..n {
                        assert_eq!(ids[u] == ids[v], reachable[u][v] && reachable[v][u]);
                    }
                    for &v in &graph[u] {
                        if ids[u] != ids[v] {
                            assert!(ids[u] < ids[v]);
                            assert!(dag[ids[u]].contains(&ids[v]));
                        }
                    }
                }
                for (from, neighbors) in dag.iter().enumerate() {
                    for &to in neighbors {
                        assert!(from < to);
                        assert!(result.clusters()[from]
                            .iter()
                            .any(|&u| { graph[u].iter().any(|&v| ids[v] == to) }));
                    }
                }
            }
        }
    }

    #[test]
    fn duplicate_edges_are_removed() {
        let graph = vec![vec![1, 2, 2], vec![0, 2], vec![]];
        assert_eq!(
            Tarjan::new(&graph).condensation_graph(),
            vec![vec![1], vec![]]
        );
    }

    #[test]
    fn deep_chain_and_cycle() {
        let n = 200_000;
        let mut graph = vec![Vec::new(); n];
        for v in 0..n - 1 {
            graph[v].push(v + 1);
        }
        let chain = Tarjan::new(&graph);
        assert_eq!(chain.clusters().len(), n);
        assert!(chain
            .get_topological_ids()
            .iter()
            .enumerate()
            .all(|(v, &id)| v == id));
        drop(chain);
        graph[n - 1].push(0);
        let cycle = Tarjan::new(&graph);
        assert_eq!(cycle.clusters().len(), 1);
        assert!(cycle.get_topological_ids().iter().all(|&id| id == 0));
    }
}
