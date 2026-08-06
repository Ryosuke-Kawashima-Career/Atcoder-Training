fn main() {
    let mut ek = EdmondsKarp::new(4);
    ek.add_edge(0, 1, 1000); // S -> A
    ek.add_edge(0, 2, 1000); // S -> B
    ek.add_edge(1, 2, 1); // A -> B
    ek.add_edge(1, 3, 1000); // A -> T
    ek.add_edge(2, 3, 1000); // B -> T
    println!("Maximum Flow: {}", ek.max_flow(0, 3)); // Output: 2000
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize,
    cap: i64,
    rev_idx: usize,
}

struct EdmondsKarp {
    graph: Vec<Vec<Edge>>,
}

impl EdmondsKarp {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let prev_idx_from: usize = self.graph[to].len();
        let prev_idx_to: usize = self.graph[from].len();
        self.graph[from].push(Edge {
            to: to,
            cap: cap,
            rev_idx: prev_idx_from,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            rev_idx: prev_idx_to,
        });
    }

    fn bfs_min_path(&self, parent: &mut Vec<Option<(usize, usize)>>, source: usize, sink: usize) {
        /* Calculate the minimum path by updating the parent of each node */
        let n: usize = self.graph.len();
        let mut visited: Vec<bool> = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        while let Some(current) = queue.pop_front() {
            visited[current] = true;
            for (idx, edge) in self.graph[current].iter().enumerate() {
                if edge.cap > 0 && !visited[edge.to] {
                    parent[edge.to] = Some((current, idx));
                    queue.push_back(edge.to);
                }
            }
        }
    }

    fn get_bottleneck_flow(&self, parent: &Vec<Option<(usize, usize)>>, sink: usize) -> i64 {
        /* Calculate the bottleneck flow by updating the parent of each node
        parent[v] = Some((u, edge_idx))
        */
        let mut current: usize = sink;
        let mut push = i64::MAX;
        while let Some((prev, edge_idx)) = parent[current] {
            push = push.min(self.graph[prev][edge_idx].cap);
            current = prev;
        }
        push
    }

    fn max_flow(&mut self, source: usize, sink: usize) -> i64 {
        let n: usize = self.graph.len();
        let mut total_flow: i64 = 0;

        loop {
            // parent[v] = Some((prev_node, edge_idx))
            let mut parent: Vec<Option<(usize, usize)>> = vec![None; n];
            self.bfs_min_path(&mut parent, source, sink);
            if parent[sink].is_none() {
                break;
            }
            let push = self.get_bottleneck_flow(&parent, sink);
            // update the graph
            let mut current: usize = sink;
            // prev -> current: edge_idx
            // current -> prev: rev_idx
            while let Some((prev, edge_idx)) = parent[current] {
                let rev_idx: usize = self.graph[prev][edge_idx].rev_idx;
                self.graph[prev][edge_idx].cap -= push;
                self.graph[current][rev_idx].cap += push;
                current = prev;
            }
            total_flow += push;
        }

        total_flow
    }
}
