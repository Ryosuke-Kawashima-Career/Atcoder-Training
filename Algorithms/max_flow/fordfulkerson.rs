fn main() {
    let mut ff = FordFulkerson::new(4);
    ff.add_edge(0, 1, 10); // S -> A
    ff.add_edge(0, 2, 10); // S -> B
    ff.add_edge(1, 2, 10); // A -> B
    ff.add_edge(1, 3, 10); // A -> T
    ff.add_edge(2, 3, 10); // B -> T
    println!("Maximum Flow: {}", ff.max_flow(0, 3));
}

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: i64,
    // Index in the adjacency list of the destination vertex
    rev_idx: usize,
}

struct FordFulkerson {
    graph: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

impl FordFulkerson {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            used: vec![false; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let rev_idx_from: usize = self.graph[to].len();
        let rev_idx_to: usize = self.graph[from].len();
        self.graph[from].push(Edge {
            to: to,
            cap: cap,
            rev_idx: rev_idx_from,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            rev_idx: rev_idx_to,
        })
    }

    fn dfs(&mut self, current: usize, goal: usize, current_flow: i64) -> i64 {
        /*Finds an increasing path and lets the flow go through it.
        Args:
            current: current vertex
            goal: goal vertex
            current_flow: current bottleneck capacity
        Returns:
            flow pushed through the path
        */
        if self.used[current] {
            return 0;
        }
        if current == goal {
            return current_flow;
        }
        self.used[current] = true;
        for edge_idx in 0..self.graph[current].len() {
            let (next_v, next_cap, rev_idx) = (
                self.graph[current][edge_idx].to,
                self.graph[current][edge_idx].cap,
                self.graph[current][edge_idx].rev_idx,
            );
            if !self.used[next_v] && next_cap > 0 {
                let next_flow: i64 = self.dfs(next_v, goal, current_flow.min(next_cap));
                if next_flow > 0 {
                    self.graph[current][edge_idx].cap -= next_flow;
                    self.graph[next_v][rev_idx].cap += next_flow;
                    return next_flow;
                }
            }
        }
        0
    }

    fn max_flow(&mut self, start: usize, goal: usize) -> i64 {
        /* Calculates the maximum flow by pushing as much flow as possible through augmenting paths
        Point of implementation: Initialize the first capacity as Infinity.
        */
        let mut total_flow: i64 = 0;
        loop {
            self.used.fill(false);
            let current_flow: i64 = self.dfs(start, goal, i64::MAX);
            if current_flow == 0 {
                break;
            }
            total_flow += current_flow;
        }
        total_flow
    }
}
